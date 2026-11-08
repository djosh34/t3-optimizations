//! What phones should have, as sync objects: the forked t3 server's state
//! cut into pieces that sync on their own.
//!
//! - Shell (`shell`, `project`, `thread` meta): the thread list, one object
//!   per project and per listed thread, and a header.
//! - Body (`c` = a projection collection): one entity of a thread's
//!   projection, such as a run, a turn item or a message, as t3's JSON. A
//!   change of `updatedAt` alone is not sent: streaming bumps it on every
//!   update.
//! - Text: the text of a message, from its turn item and its message alike
//!   (keyed by message id), or of a reasoning item. Bodies carry `"text":""`
//!   and the phone fills it in from here. While the agent writes, the raw
//!   partial text from `orchestration.subscribeLiveText` takes the place of
//!   the buffered text t3 stores.
//! - Frame: a thread's snapshot fields and its visible rows, with local rows'
//!   items left out (they are bodies).
//! - Image: an attachment's bytes.
//! - Config: the server's config (`server.getConfig`), its environment
//!   descriptor and the sidecar's auth session on it, which the web app
//!   needs before it shows anything.
//!
//! Every object has a `meta` JSON that says what it is; see `crates/sidecar/README.md`.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use sync_core::MAX_META;

/// An object's kind decides its class (see `crate::device`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Config,
    Shell,
    Body,
    Text,
    Frame,
    Image,
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub thread: Option<String>,
    pub kind: Kind,
    /// In the phone's current thread, a body that goes with approvals and
    /// status rather than with older messages.
    pub status: bool,
    /// Higher goes first within its class.
    pub order: u64,
    pub meta: Vec<u8>,
    /// Empty for a deleted object.
    pub content: Vec<u8>,
}

const CONFIG: &str = "config";
const DESCRIPTOR: &str = "descriptor";
const SESSION: &str = "session";

/// Projection collections whose bodies go with status in the current
/// thread. Turn items that are not finished go there too.
const STATUS_COLLECTIONS: &[&str] = &[
    "thread",
    "runs",
    "attempts",
    "runtimeRequests",
    "plans",
    "subagents",
];

/// Which collection an event updates (see client-runtime's
/// orchestrationV2Projection.ts).
fn event_collection(event_type: &str) -> Option<&'static str> {
    Some(match event_type {
        t if t.starts_with("thread.") && t != "thread.pull-request-link-updated" => "thread",
        "run.created" | "run.updated" => "runs",
        "run-attempt.created" | "run-attempt.updated" => "attempts",
        "node.updated" => "nodes",
        "subagent.updated" => "subagents",
        "provider-session.attached" | "provider-session.updated" => "providerSessions",
        "provider-thread.updated" => "providerThreads",
        "provider-turn.updated" => "providerTurns",
        "runtime-request.updated" => "runtimeRequests",
        "message.updated" => "messages",
        "plan.updated" => "plans",
        "turn-item.updated" => "turnItems",
        "checkpoint-scope.created" => "checkpointScopes",
        "checkpoint.captured" => "checkpoints",
        "context-handoff.updated" => "contextHandoffs",
        "context-transfer.created" | "context-transfer.updated" => "contextTransfers",
        _ => return None,
    })
}

/// What an event asks of the sidecar beyond updating an entity.
#[derive(Debug, PartialEq, Eq)]
pub enum After {
    Nothing,
    /// The event changes the projection in a way the sidecar does not
    /// apply itself; fetch the thread's snapshot again.
    Refetch,
}

#[derive(Debug, Default)]
struct ThreadState {
    /// Entities as t3 has them, by collection and id, with their text.
    entities: HashMap<(String, String), Value>,
    /// Text keys and their text as t3 stores it.
    stored_text: HashMap<String, String>,
    /// Raw text of messages the agent is still writing.
    live_text: HashMap<String, String>,
    /// The kind label of each text key.
    text_kind: HashMap<String, String>,
    /// Ordinal and text key of each assistant message, for the tail.
    assistant: BTreeMap<u64, String>,
    /// The ordinal of the newest user message: the current turn starts
    /// there.
    turn_start: u64,
    /// The ordinal of the turn item behind each turn item body and text
    /// object, by object id.
    turn_ordinals: HashMap<u64, u64>,
}

#[derive(Debug, Default)]
pub struct World {
    objects: HashMap<u64, Obj>,
    by_thread: HashMap<String, BTreeSet<u64>>,
    threads: HashMap<String, ThreadState>,
    /// Listed threads and where: thread id to location.
    shell_threads: HashMap<String, String>,
    shell_projects: HashSet<String>,
    changed: BTreeSet<u64>,
    /// The order of things with no better one: later is newer.
    seq: u64,
    images: HashSet<String>,
    wanted_images: Vec<(String, String)>,
    /// What a sidecar that just started has rebuilt: the thread list, the
    /// threads that had a snapshot, the config and the descriptor.
    shell_loaded: bool,
    loaded_threads: HashSet<String>,
    /// Ids whose meta did not fit, reported once.
    pub skipped: Vec<String>,
}

/// The object id of a list of names.
pub fn object_id(parts: &[&str]) -> u64 {
    let mut h = Sha256::new();
    for p in parts {
        h.update(p.as_bytes());
        h.update([0]);
    }
    let digest = h.finalize();
    let mut first = [0u8; 8];
    first.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(first) & sync_core::wire::MAX_ID
}

fn compact(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).unwrap_or_default()
}

impl World {
    pub fn get(&self, id: u64) -> Option<&Obj> {
        self.objects.get(&id)
    }

    pub fn ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.objects.keys().copied()
    }

    pub fn thread_ids(&self, thread: &str) -> impl Iterator<Item = u64> + '_ {
        self.by_thread.get(thread).into_iter().flatten().copied()
    }

    /// Objects changed since the last call.
    pub fn take_changed(&mut self) -> Vec<u64> {
        std::mem::take(&mut self.changed).into_iter().collect()
    }

    /// Attachments to fetch: id and MIME type.
    pub fn take_wanted_images(&mut self) -> Vec<(String, String)> {
        std::mem::take(&mut self.wanted_images)
    }

    /// The text object of the thread's newest assistant message.
    pub fn tail(&self, thread: &str) -> Option<u64> {
        let key = self.threads.get(thread)?.assistant.values().next_back()?;
        Some(object_id(&["text", thread, key]))
    }

    /// Whether an object is a turn item, or its text, of the thread's
    /// current turn: from its newest user message on.
    pub fn in_current_turn(&self, thread: &str, id: u64) -> bool {
        self.threads
            .get(thread)
            .and_then(|s| Some(*s.turn_ordinals.get(&id)? >= s.turn_start))
            .unwrap_or(false)
    }

    /// The listed threads.
    pub fn listed_threads(&self) -> impl Iterator<Item = &String> {
        self.shell_threads.keys()
    }

    fn next_seq(&mut self) -> u64 {
        self.seq += 1;
        self.seq
    }

    /// Sets an object; marks it changed when it is new or differs.
    fn put(&mut self, id: u64, obj: Obj) {
        if obj.meta.len() > MAX_META {
            self.skipped
                .push(String::from_utf8_lossy(&obj.meta).into_owned());
            return;
        }
        if let Some(old) = self.objects.get(&id)
            && old.content == obj.content
            && old.meta == obj.meta
            && old.status == obj.status
            && old.order == obj.order
        {
            return;
        }
        if let Some(t) = &obj.thread {
            self.by_thread.entry(t.clone()).or_default().insert(id);
        }
        self.objects.insert(id, obj);
        self.changed.insert(id);
    }

    /// Empties an object: the phone deletes what it had.
    fn delete(&mut self, id: u64) {
        if let Some(obj) = self.objects.get_mut(&id)
            && !obj.content.is_empty()
        {
            obj.content.clear();
            self.changed.insert(id);
        }
    }

    /// Whether everything the forked server has is here: the thread list,
    /// a snapshot of every listed thread, the config, the descriptor and the
    /// session.
    /// Until then, an object the world lacks may just not be loaded yet.
    pub fn complete(&self) -> bool {
        self.shell_loaded
            && self
                .shell_threads
                .keys()
                .all(|t| self.loaded_threads.contains(t))
            && [CONFIG, DESCRIPTOR, SESSION]
                .iter()
                .all(|c| self.objects.contains_key(&object_id(&[c])))
    }

    /// The objects the world has or will have: images it asked for count.
    /// Objects a phone has beyond these were deleted.
    pub fn known_ids(&self) -> HashSet<u64> {
        let images = self.images.iter().map(|a| object_id(&["image", a]));
        self.objects
            .iter()
            .filter(|(_, o)| !o.content.is_empty())
            .map(|(id, _)| *id)
            .chain(images)
            .collect()
    }

    // The config.

    /// The server's config, from `server.getConfig`.
    pub fn set_config(&mut self, config: &Value) {
        self.put_config(CONFIG, compact(config));
    }

    /// The environment descriptor, from `/.well-known/t3/environment`.
    pub fn set_descriptor(&mut self, descriptor: &Value) {
        self.put_config(DESCRIPTOR, compact(descriptor));
    }

    /// The sidecar's session on the server, from `/api/auth/session`: the
    /// web app shows the scopes and permissions it has through the sidecar.
    pub fn set_session(&mut self, session: &Value) {
        self.put_config(SESSION, compact(session));
    }

    fn put_config(&mut self, c: &str, content: Vec<u8>) {
        self.put(
            object_id(&[c]),
            Obj {
                thread: None,
                kind: Kind::Config,
                status: false,
                order: u64::MAX,
                meta: compact(&json!({ "c": c })),
                content,
            },
        );
    }

    // The thread list.

    /// A `subscribeShell` snapshot item.
    pub fn set_shell(&mut self, item: &Value) {
        let snap = &item["snapshot"];
        // A snapshot with resolved roots only refreshes the projects'
        // repository identity; its empty thread list means nothing (see
        // client-runtime's mergeShellSnapshotProjects).
        if item.get("resolvedRepositoryIdentityRoots").is_some() {
            for p in snap["projects"].as_array().into_iter().flatten() {
                if let Some(id) = p["id"].as_str()
                    && self.shell_projects.contains(id)
                {
                    self.put_project(id, p);
                }
            }
            return;
        }
        self.shell_loaded = true;
        let header = json!({
            "schemaVersion": snap["schemaVersion"],
            "snapshotSequence": snap["snapshotSequence"],
            "olderSettled": snap.get("olderSettled"),
        });
        self.put(
            object_id(&["shell"]),
            Obj {
                thread: None,
                // The web app shows nothing before it has the header.
                kind: Kind::Config,
                status: false,
                order: u64::MAX,
                meta: compact(&json!({"c": "shell"})),
                content: compact(&header),
            },
        );
        let mut threads = HashMap::new();
        for (location, key) in [("active", "threads"), ("archive", "archivedThreads")] {
            for t in snap[key].as_array().into_iter().flatten() {
                if let Some(id) = t["id"].as_str() {
                    threads.insert(id.to_owned(), (location.to_owned(), t.clone()));
                }
            }
        }
        let gone: Vec<String> = self
            .shell_threads
            .keys()
            .filter(|id| !threads.contains_key(*id))
            .cloned()
            .collect();
        for id in gone {
            self.remove_shell_thread(&id);
        }
        for (id, (location, t)) in threads {
            self.put_shell_thread(&id, &location, &t);
        }
        let projects: HashMap<String, Value> = snap["projects"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|p| Some((p["id"].as_str()?.to_owned(), p.clone())))
            .collect();
        let gone: Vec<String> = self
            .shell_projects
            .iter()
            .filter(|id| !projects.contains_key(*id))
            .cloned()
            .collect();
        for id in gone {
            self.shell_projects.remove(&id);
            self.delete(object_id(&["project", &id]));
        }
        for (id, p) in projects {
            self.put_project(&id, &p);
        }
    }

    /// A `subscribeShell` item after the snapshot.
    pub fn apply_shell(&mut self, item: &Value) {
        match item["kind"].as_str() {
            Some("snapshot") => self.set_shell(item),
            Some("project.updated") => {
                if let Some(id) = item["project"]["id"].as_str() {
                    self.put_project(id, &item["project"]);
                }
            }
            Some("project.removed") => {
                if let Some(id) = item["projectId"].as_str() {
                    self.shell_projects.remove(id);
                    self.delete(object_id(&["project", id]));
                }
            }
            Some("thread.updated") => {
                let t = &item["thread"];
                if let (Some(id), Some(location)) = (t["id"].as_str(), item["location"].as_str()) {
                    self.put_shell_thread(id, location, t);
                }
            }
            Some("thread.removed") => {
                if let Some(id) = item["threadId"].as_str() {
                    self.remove_shell_thread(id);
                }
            }
            _ => {}
        }
    }

    fn put_project(&mut self, id: &str, project: &Value) {
        self.shell_projects.insert(id.to_owned());
        self.put(
            object_id(&["project", id]),
            Obj {
                thread: None,
                // The open thread's view needs its project.
                kind: Kind::Config,
                status: false,
                order: u64::MAX,
                meta: compact(&json!({"c": "project", "i": id})),
                content: compact(project),
            },
        );
    }

    fn put_shell_thread(&mut self, id: &str, location: &str, thread: &Value) {
        if let Some(old) = self.shell_threads.get(id)
            && old != location
        {
            self.delete(object_id(&["thread", old, id]));
        }
        self.shell_threads
            .insert(id.to_owned(), location.to_owned());
        let order = self.next_seq();
        self.put(
            object_id(&["thread", location, id]),
            Obj {
                // The thread's own: its row goes with its status while it is open.
                thread: Some(id.to_owned()),
                kind: Kind::Shell,
                status: false,
                order,
                meta: compact(&json!({"c": "thread", "i": id, "l": location})),
                content: compact(thread),
            },
        );
    }

    fn remove_shell_thread(&mut self, id: &str) {
        if let Some(location) = self.shell_threads.remove(id) {
            self.delete(object_id(&["thread", &location, id]));
        }
        // A thread that is gone from the list is gone: delete its content.
        let ids: Vec<u64> = self.thread_ids(id).collect();
        for oid in ids {
            self.delete(oid);
        }
        self.threads.remove(id);
    }

    // A thread's content.

    /// A thread's snapshot: a `subscribeThread` snapshot item or t3's HTTP
    /// thread snapshot. Entities it does not have any more are deleted.
    pub fn set_thread(&mut self, thread: &str, snapshot: &Value) {
        let projection = &snapshot["projection"];
        let Some(fields) = projection.as_object() else {
            return;
        };
        let mut seen: HashSet<(String, String)> = HashSet::new();
        for (coll, value) in fields {
            if coll == "visibleTurnItems" {
                continue;
            }
            if coll == "thread" {
                seen.insert(("thread".into(), "thread".into()));
                self.put_entity(thread, "thread", value);
                continue;
            }
            for entity in value.as_array().into_iter().flatten() {
                if let Some(id) = entity["id"].as_str() {
                    seen.insert((coll.clone(), id.to_owned()));
                    self.put_entity(thread, coll, entity);
                }
            }
        }
        self.loaded_threads.insert(thread.to_owned());
        let complete = snapshot["hasMoreHistory"].as_bool() != Some(true);
        if complete {
            let gone: Vec<(String, String)> = self
                .threads
                .get(thread)
                .map(|t| {
                    t.entities
                        .iter()
                        // An item the agent is still writing may exist
                        // only in live frames: t3 has not stored it, so a
                        // snapshot lacks it.
                        .filter(|((coll, _), e)| {
                            !text_key(coll, e).is_some_and(|k| t.live_text.contains_key(&k))
                        })
                        .map(|(k, _)| k)
                        .filter(|k| !seen.contains(*k))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();
            for (coll, id) in gone {
                self.remove_entity(thread, &coll, &id);
            }
        }
        self.put_frame(thread, snapshot);
    }

    /// One `subscribeThread` event item.
    pub fn apply_event(&mut self, thread: &str, item: &Value) -> After {
        let event = &item["event"];
        let Some(event_type) = event["type"].as_str() else {
            return After::Nothing;
        };
        let payload = &event["payload"];
        if event_type == "provider-session.detached" {
            if let Some(id) = payload["providerSessionId"].as_str() {
                self.remove_entity(thread, "providerSessions", id);
            }
            return After::Nothing;
        }
        if event_type == "checkpoint.rollback-requested" {
            return After::Nothing;
        }
        match event_collection(event_type) {
            Some(coll) => {
                self.put_entity(thread, coll, payload);
                // A new turn item may change the visible rows; the phone
                // places it itself, the frame catches up on the next fetch.
                After::Nothing
            }
            None => After::Refetch,
        }
    }

    /// One `subscribeLiveText` frame: an assistant or reasoning item with
    /// its raw text.
    pub fn apply_live(&mut self, frame: &Value) {
        let item = &frame["item"];
        let (Some(thread), Some(text)) = (item["threadId"].as_str(), item["text"].as_str()) else {
            return;
        };
        let Some(key) = text_key("turnItems", item) else {
            return;
        };
        let streaming = item["streaming"].as_bool() == Some(true);
        let state = self.threads.entry(thread.to_owned()).or_default();
        if streaming {
            state.live_text.insert(key.clone(), text.to_owned());
        } else {
            state.live_text.remove(&key);
            // The finished text; t3 stores the same.
            state.stored_text.insert(key.clone(), text.to_owned());
        }
        // An item t3 holds back has no stored event yet: its body comes
        // from here until it has.
        let known = item["id"].as_str().is_some_and(|id| {
            state
                .entities
                .contains_key(&("turnItems".into(), id.to_owned()))
        });
        if !known {
            // A live item carries the provider's own ordinal (201, say),
            // not t3's (2000002), which t3 gives it when it stores it. It is
            // the item being written now, so it goes after every stored one
            // until then; otherwise an older message stayed the tail and the
            // live text went out behind everything else.
            let after = state
                .entities
                .iter()
                .filter(|((coll, _), _)| coll == "turnItems")
                .filter_map(|(_, e)| e["ordinal"].as_u64())
                .max()
                .map_or(0, |o| o + 1);
            let mut item = item.clone();
            if let Some(o) = item.as_object_mut() {
                o.insert("ordinal".into(), after.into());
            }
            self.put_entity(thread, "turnItems", &item);
        } else {
            // Its place is t3's, from the stored item.
            self.put_text(thread, &key);
        }
    }

    fn put_entity(&mut self, thread: &str, coll: &str, entity: &Value) {
        let id = if coll == "thread" {
            "thread".to_owned()
        } else {
            match entity["id"].as_str() {
                Some(id) => id.to_owned(),
                None => return,
            }
        };
        let key = text_key(coll, entity);
        let mut body = entity.clone();
        let mut text = None;
        if key.is_some()
            && let Some(obj) = body.as_object_mut()
            && let Some(Value::String(t)) = obj.get_mut("text")
        {
            text = Some(std::mem::take(t));
        }
        let state = self.threads.entry(thread.to_owned()).or_default();
        let entity_key = (coll.to_owned(), id.clone());
        let unchanged = state
            .entities
            .get(&entity_key)
            .is_some_and(|old| same_but_time_and_text(old, entity));
        state.entities.insert(entity_key, entity.clone());
        if let (Some(key), Some(text)) = (&key, text) {
            let finished = entity["streaming"].as_bool() == Some(false);
            if finished {
                state.live_text.remove(key);
            }
            state.stored_text.insert(key.clone(), text);
            state.text_kind.insert(key.clone(), text_kind(coll, entity));
            if coll == "turnItems" {
                self.note_assistant(thread, entity, key);
                self.note_turn(thread, entity, object_id(&["text", thread, key]));
            }
            self.put_text(thread, key);
        }
        if coll == "turnItems" {
            self.note_turn(thread, entity, object_id(&["body", thread, coll, &id]));
        }
        if unchanged {
            return;
        }
        let order = match entity["ordinal"].as_u64() {
            Some(o) if coll == "turnItems" => o,
            _ => self.next_seq(),
        };
        let status = STATUS_COLLECTIONS.contains(&coll)
            || (coll == "turnItems"
                && matches!(
                    entity["status"].as_str(),
                    Some("pending" | "running" | "waiting")
                ));
        self.put(
            object_id(&["body", thread, coll, &id]),
            Obj {
                thread: Some(thread.to_owned()),
                kind: Kind::Body,
                status,
                order,
                meta: compact(&json!({"t": thread, "c": coll, "i": id})),
                content: compact(&body),
            },
        );
        if coll == "turnItems" {
            self.want_images(entity);
        }
    }

    fn remove_entity(&mut self, thread: &str, coll: &str, id: &str) {
        self.delete(object_id(&["body", thread, coll, id]));
        let Some(state) = self.threads.get_mut(thread) else {
            return;
        };
        let Some(entity) = state.entities.remove(&(coll.to_owned(), id.to_owned())) else {
            return;
        };
        // Its text goes too once no other entity shows it (a message and
        // its turn item share one), and a deleted message is no tail.
        let Some(key) = text_key(coll, &entity) else {
            return;
        };
        let shown = state
            .entities
            .iter()
            .any(|((c, _), e)| text_key(c, e).as_deref() == Some(key.as_str()));
        if shown {
            return;
        }
        state.stored_text.remove(&key);
        state.live_text.remove(&key);
        state.text_kind.remove(&key);
        let old_tail = state.assistant.values().next_back().cloned();
        state.assistant.retain(|_, k| *k != key);
        let new_tail = state.assistant.values().next_back().cloned();
        if old_tail != new_tail {
            for k in [old_tail, new_tail].into_iter().flatten() {
                self.changed.insert(object_id(&["text", thread, &k]));
            }
        }
        self.delete(object_id(&["text", thread, &key]));
    }

    /// Remembers an assistant message's place for the tail.
    fn note_assistant(&mut self, thread: &str, item: &Value, key: &str) {
        if item["type"].as_str() != Some("assistant_message") {
            return;
        }
        let Some(ordinal) = item["ordinal"].as_u64() else {
            return;
        };
        let state = self.threads.entry(thread.to_owned()).or_default();
        let old_tail = state.assistant.values().next_back().cloned();
        // t3's ordinal takes the place of a provisional one.
        state.assistant.retain(|_, k| k != key);
        state.assistant.insert(ordinal, key.to_owned());
        let new_tail = state.assistant.values().next_back().cloned();
        if old_tail != new_tail {
            // Both change class.
            for k in [old_tail, new_tail].into_iter().flatten() {
                self.changed.insert(object_id(&["text", thread, &k]));
            }
        }
    }

    /// Notes the turn item behind an object, and a new turn when the item
    /// is a newer user message: then the previous turn's objects change
    /// class.
    fn note_turn(&mut self, thread: &str, item: &Value, id: u64) {
        let Some(ordinal) = item["ordinal"].as_u64() else {
            return;
        };
        let state = self.threads.entry(thread.to_owned()).or_default();
        state.turn_ordinals.insert(id, ordinal);
        if item["type"].as_str() == Some("user_message") && ordinal > state.turn_start {
            state.turn_start = ordinal;
            self.changed.extend(state.turn_ordinals.keys().copied());
        }
    }

    fn put_text(&mut self, thread: &str, key: &str) {
        let Some(state) = self.threads.get(thread) else {
            return;
        };
        let Some(text) = state
            .live_text
            .get(key)
            .or_else(|| state.stored_text.get(key))
            .cloned()
        else {
            return;
        };
        let kind = state.text_kind.get(key).cloned().unwrap_or_default();
        let order = state
            .assistant
            .iter()
            .find(|(_, k)| *k == key)
            .map(|(o, _)| *o);
        let order = match order {
            Some(o) => o,
            None => self.next_seq(),
        };
        let id = object_id(&["text", thread, key]);
        // The order of a text the phone already has stays, so it is not
        // sent again for a new order.
        let order = self.objects.get(&id).map_or(order, |o| o.order);
        self.put(
            id,
            Obj {
                thread: Some(thread.to_owned()),
                kind: Kind::Text,
                status: false,
                order,
                meta: compact(&json!({"t": thread, "m": key, "k": kind})),
                content: text.into_bytes(),
            },
        );
    }

    fn put_frame(&mut self, thread: &str, snapshot: &Value) {
        let rows: Vec<Value> = snapshot["projection"]["visibleTurnItems"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|row| {
                let mut row = row.clone();
                if row["visibility"].as_str() == Some("local")
                    && let Some(obj) = row.as_object_mut()
                {
                    obj.remove("item");
                }
                row
            })
            .collect();
        let mut frame = Map::new();
        for key in [
            "snapshotSequence",
            "historyCursor",
            "hasMoreHistory",
            "latestLocalTurnOrdinal",
        ] {
            if let Some(v) = snapshot.get(key) {
                frame.insert(key.into(), v.clone());
            }
        }
        frame.insert(
            "updatedAt".into(),
            snapshot["projection"]["updatedAt"].clone(),
        );
        frame.insert("visibleTurnItems".into(), Value::Array(rows));
        self.put(
            object_id(&["frame", thread]),
            Obj {
                thread: Some(thread.to_owned()),
                kind: Kind::Frame,
                status: true,
                order: u64::MAX,
                meta: compact(&json!({"t": thread, "c": "frame"})),
                content: compact(&Value::Object(frame)),
            },
        );
    }

    // Images.

    fn want_images(&mut self, item: &Value) {
        for a in item["attachments"].as_array().into_iter().flatten() {
            if a["type"].as_str() != Some("image") {
                continue;
            }
            let (Some(id), Some(mime)) = (a["id"].as_str(), a["mimeType"].as_str()) else {
                continue;
            };
            if self.images.insert(id.to_owned()) {
                self.wanted_images.push((id.to_owned(), mime.to_owned()));
            }
        }
    }

    pub fn set_image(&mut self, id: &str, mime: &str, bytes: Vec<u8>) {
        let order = self.next_seq();
        self.put(
            object_id(&["image", id]),
            Obj {
                thread: None,
                kind: Kind::Image,
                status: false,
                order,
                meta: compact(&json!({"c": "image", "i": id, "y": mime})),
                content: bytes,
            },
        );
    }

    /// Forgets that an image was asked for, so it is asked for again.
    pub fn image_failed(&mut self, id: &str) {
        self.images.remove(id);
    }
}

/// The key of an entity's text: the message id for messages and the turn
/// items that show them, the item id for other items with text.
fn text_key(coll: &str, entity: &Value) -> Option<String> {
    entity.get("text")?.as_str()?;
    match coll {
        "messages" => entity["id"].as_str().map(str::to_owned),
        "turnItems" => entity["messageId"]
            .as_str()
            .or_else(|| entity["id"].as_str())
            .map(str::to_owned),
        _ => None,
    }
}

/// A short label of what a text is: a message's role, or the item type
/// for other items.
fn text_kind(coll: &str, entity: &Value) -> String {
    let label = if coll == "messages" {
        entity["role"].as_str()
    } else {
        match entity["type"].as_str() {
            Some("assistant_message") => Some("assistant"),
            Some("user_message") => Some("user"),
            other => other,
        }
    };
    label.unwrap_or("text").to_owned()
}

/// Whether two versions of an entity differ only in `updatedAt` and text.
fn same_but_time_and_text(a: &Value, b: &Value) -> bool {
    let strip = |v: &Value| {
        let mut v = v.clone();
        if let Some(o) = v.as_object_mut() {
            o.remove("updatedAt");
            o.remove("text");
        }
        v
    };
    strip(a) == strip(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, ordinal: u64, text: &str, streaming: bool, updated: &str) -> Value {
        json!({
            "id": id, "threadId": "t1", "type": "assistant_message", "messageId": format!("m-{id}"),
            "ordinal": ordinal, "status": if streaming { "running" } else { "completed" },
            "text": text, "streaming": streaming, "updatedAt": updated,
        })
    }

    fn event(event_type: &str, payload: Value) -> Value {
        json!({"kind": "event", "sequence": 1, "event": {"type": event_type, "threadId": "t1", "payload": payload}})
    }

    fn content(w: &World, id: u64) -> String {
        String::from_utf8(w.get(id).map(|o| o.content.clone()).unwrap_or_default())
            .unwrap_or_default()
    }

    #[test]
    fn text_is_its_own_object_and_time_alone_is_not_a_change() {
        let mut w = World::default();
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("a", 5, "Hello", true, "1")),
        );
        let body = object_id(&["body", "t1", "turnItems", "a"]);
        let text = object_id(&["text", "t1", "m-a"]);
        assert!(content(&w, body).contains(r#""text":"""#));
        assert_eq!(content(&w, text), "Hello");
        assert_eq!(w.tail("t1"), Some(text));
        w.take_changed();

        // More text and a new updatedAt: only the text changes.
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("a", 5, "Hello world", true, "2")),
        );
        assert_eq!(w.take_changed(), vec![text]);
        assert_eq!(content(&w, text), "Hello world");
    }

    #[test]
    fn live_text_leads_until_the_message_is_finished() {
        let mut w = World::default();
        let text = object_id(&["text", "t1", "m-a"]);
        // Held back by t3: only live text, and the item from it.
        w.apply_live(&json!({"item": item("a", 5, "An open para", true, "1")}));
        assert_eq!(content(&w, text), "An open para");
        assert!(
            w.get(object_id(&["body", "t1", "turnItems", "a"]))
                .is_some()
        );
        // t3's buffered text is shorter; live text stays.
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("a", 5, "An", true, "2")),
        );
        w.apply_live(&json!({"item": item("a", 5, "An open paragraph", true, "3")}));
        assert_eq!(content(&w, text), "An open paragraph");
        // Finished: t3's text.
        w.apply_event(
            "t1",
            &event(
                "turn-item.updated",
                item("a", 5, "An open paragraph.", false, "4"),
            ),
        );
        assert_eq!(content(&w, text), "An open paragraph.");
    }

    #[test]
    fn a_live_item_is_the_tail_before_t3_stores_it() {
        let mut w = World::default();
        w.apply_event(
            "t1",
            &event(
                "turn-item.updated",
                item("old", 1_000_003, "Earlier reply.", false, "1"),
            ),
        );
        // The provider's own ordinal is far below t3's.
        w.apply_live(&json!({"item": item("new", 201, "The sync loop starts", true, "2")}));
        let new = object_id(&["text", "t1", "m-new"]);
        assert_eq!(w.tail("t1"), Some(new));
        // More live text, then t3 stores it with its own ordinal.
        w.apply_live(&json!({"item": item("new", 201, "The sync loop starts from", true, "3")}));
        assert_eq!(w.tail("t1"), Some(new));
        w.apply_event(
            "t1",
            &event(
                "turn-item.updated",
                item("new", 2_000_002, "The sync", true, "4"),
            ),
        );
        assert_eq!(w.tail("t1"), Some(new));
        assert_eq!(content(&w, new), "The sync loop starts from");
    }

    #[test]
    fn the_current_turn_starts_at_the_newest_user_message() {
        let mut w = World::default();
        let user = |id: &str, ordinal: u64| {
            json!({"id": id, "threadId": "t1", "type": "user_message", "messageId": format!("m-{id}"),
                "ordinal": ordinal, "status": "completed", "text": "prompt", "streaming": false, "updatedAt": "1"})
        };
        let body = |id: &str| object_id(&["body", "t1", "turnItems", id]);
        let text = |id: &str| object_id(&["text", "t1", &format!("m-{id}")]);
        w.apply_event("t1", &event("turn-item.updated", user("u1", 1)));
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("a", 2, "first", false, "1")),
        );
        w.apply_event("t1", &event("turn-item.updated", user("u2", 3)));
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("b", 4, "second", true, "1")),
        );
        for id in ["u2", "b"] {
            assert!(w.in_current_turn("t1", body(id)), "{id} body");
            assert!(w.in_current_turn("t1", text(id)), "{id} text");
        }
        for id in ["u1", "a"] {
            assert!(!w.in_current_turn("t1", body(id)), "{id} body");
            assert!(!w.in_current_turn("t1", text(id)), "{id} text");
        }
    }

    #[test]
    fn the_tail_moves_to_the_newest_assistant_message() {
        let mut w = World::default();
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("a", 5, "first", false, "1")),
        );
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("b", 7, "second", true, "1")),
        );
        assert_eq!(w.tail("t1"), Some(object_id(&["text", "t1", "m-b"])));
    }

    #[test]
    fn a_snapshot_deletes_entities_it_no_longer_has() {
        let mut w = World::default();
        let snap = |items: Value| {
            json!({"snapshotSequence": 3, "projection": {
                "thread": {"id": "t1", "title": "x"}, "turnItems": items, "visibleTurnItems": [], "updatedAt": "now"}})
        };
        w.set_thread(
            "t1",
            &snap(json!([
                item("a", 1, "x", false, "1"),
                item("b", 2, "y", false, "1")
            ])),
        );
        let b = object_id(&["body", "t1", "turnItems", "b"]);
        assert!(!content(&w, b).is_empty());
        w.set_thread("t1", &snap(json!([item("a", 1, "x", false, "1")])));
        assert!(content(&w, b).is_empty());
        assert!(!content(&w, object_id(&["frame", "t1"])).is_empty());
    }

    #[test]
    fn an_enrichment_snapshot_keeps_the_threads() {
        let mut w = World::default();
        let project = json!({"id": "p1", "workspaceRoot": "/w", "repositoryIdentity": null});
        w.set_shell(&json!({"kind": "snapshot", "snapshot": {
            "schemaVersion": 2, "snapshotSequence": 5, "projects": [project],
            "threads": [{"id": "t1", "title": "x"}], "archivedThreads": []}}));
        let listed = object_id(&["thread", "active", "t1"]);
        assert!(!content(&w, listed).is_empty());
        let enriched =
            json!({"id": "p1", "workspaceRoot": "/w", "repositoryIdentity": {"name": "r"}});
        w.set_shell(&json!({"kind": "snapshot", "resolvedRepositoryIdentityRoots": ["/w"], "snapshot": {
            "schemaVersion": 2, "snapshotSequence": 5, "projects": [enriched], "threads": [], "archivedThreads": []}}));
        assert!(!content(&w, listed).is_empty());
        assert!(content(&w, object_id(&["project", "p1"])).contains(r#""name":"r""#));
    }

    #[test]
    fn a_deleted_message_takes_its_text_and_the_tail_with_it() {
        let mut w = World::default();
        let snap = |items: Value| {
            json!({"snapshotSequence": 3, "projection": {
                "thread": {"id": "t1"}, "turnItems": items, "visibleTurnItems": [], "updatedAt": "now"}})
        };
        w.set_thread(
            "t1",
            &snap(json!([
                item("a", 1, "first", false, "1"),
                item("b", 2, "second", false, "1")
            ])),
        );
        let (a, b) = (
            object_id(&["text", "t1", "m-a"]),
            object_id(&["text", "t1", "m-b"]),
        );
        assert_eq!(w.tail("t1"), Some(b));
        w.set_thread("t1", &snap(json!([item("a", 1, "first", false, "1")])));
        assert!(
            content(&w, b).is_empty(),
            "the deleted message's text stayed"
        );
        assert_eq!(w.tail("t1"), Some(a));
        assert_eq!(content(&w, a), "first");
    }

    #[test]
    fn a_snapshot_keeps_a_message_t3_has_not_stored_yet() {
        let mut w = World::default();
        let snap = |items: Value| {
            json!({"snapshotSequence": 3, "projection": {
                "thread": {"id": "t1"}, "turnItems": items, "visibleTurnItems": [], "updatedAt": "now"}})
        };
        w.set_thread("t1", &snap(json!([item("a", 1, "first", false, "1")])));
        w.apply_live(&json!({"item": item("b", 201, "being written", true, "2")}));
        let b = object_id(&["text", "t1", "m-b"]);
        assert_eq!(w.tail("t1"), Some(b));
        // A snapshot taken while t3 holds the message back.
        w.set_thread("t1", &snap(json!([item("a", 1, "first", false, "1")])));
        assert_eq!(w.tail("t1"), Some(b));
        assert_eq!(content(&w, b), "being written");
        assert!(!content(&w, object_id(&["body", "t1", "turnItems", "b"])).is_empty());
    }

    #[test]
    fn unknown_events_ask_for_a_refetch() {
        let mut w = World::default();
        assert_eq!(
            w.apply_event("t1", &event("thread.pull-request-link-updated", json!({}))),
            After::Refetch
        );
        assert_eq!(
            w.apply_event("t1", &event("run.updated", json!({"id": "r"}))),
            After::Nothing
        );
    }

    #[test]
    fn images_are_asked_for_once() {
        let mut w = World::default();
        let user = json!({"id": "u", "threadId": "t1", "type": "user_message", "messageId": "mu", "ordinal": 1,
            "text": "look", "attachments": [{"type": "image", "id": "img1", "mimeType": "image/png"}]});
        w.apply_event("t1", &event("turn-item.updated", user.clone()));
        w.apply_event("t1", &event("turn-item.updated", user));
        assert_eq!(
            w.take_wanted_images(),
            vec![("img1".to_owned(), "image/png".to_owned())]
        );
    }

    #[test]
    fn complete_once_every_listed_thread_and_the_config_are_here() {
        let mut w = World::default();
        let shell = |threads: Value| {
            json!({"kind": "snapshot", "snapshot": {"schemaVersion": 2, "snapshotSequence": 1,
                "projects": [], "threads": threads, "archivedThreads": []}})
        };
        let snap = json!({"snapshotSequence": 3, "projection": {
            "thread": {"id": "t1"}, "turnItems": [item("a", 1, "x", false, "1")],
            "visibleTurnItems": [], "updatedAt": "now"}});
        assert!(!w.complete());
        w.set_shell(&shell(json!([{"id": "t1"}, {"id": "t2"}])));
        w.set_config(&json!({"environment": {}}));
        w.set_descriptor(&json!({"environmentId": "e"}));
        w.set_session(&json!({"authenticated": true}));
        w.set_thread("t1", &snap);
        assert!(!w.complete(), "t2 has no snapshot yet");
        w.set_thread("t2", &snap);
        assert!(w.complete());
        // A thread that went away takes its objects out of the known ones.
        let text = object_id(&["text", "t2", "m-a"]);
        assert!(w.known_ids().contains(&text));
        w.set_shell(&shell(json!([{"id": "t1"}])));
        assert!(!w.known_ids().contains(&text));
        assert!(w.known_ids().contains(&object_id(&["text", "t1", "m-a"])));
    }
}
