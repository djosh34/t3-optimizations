//! What phones should have, as sync objects: the t3 server's state
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
//!   and the phone fills it in from here. While the agent writes, this is the
//!   text t3 has released so far: finished lines, and code blocks once they
//!   close.
//! - Frame: a thread's snapshot fields and its visible rows, with local rows'
//!   items left out (they are bodies).
//! - Image: an attachment's bytes.
//! - Config: the server's config (`server.getConfig`), its environment
//!   descriptor and the sidecar's auth session on it, which the web app
//!   needs before it shows anything.
//!
//! Every object has a `meta` JSON that says what it is; every id in it is
//! hashed with `sync_core::hash_id`. See `crates/sidecar/README.md`.
//!
//! The world holds only the threads phones opened or were sent ahead of
//! time, as encoded bytes, up to a byte limit: past it, the thread used
//! least recently goes, with the images only it showed (`World::evict_over`).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Arc;

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use sync_core::{MAX_META, hash, hash_id};

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
    /// Empty for a deleted object. Shared with every phone's sync state.
    pub content: Arc<[u8]>,
}

impl Obj {
    /// What the object costs the cache: its bytes and about what its map
    /// entries and its entity take.
    fn cost(&self) -> usize {
        self.content.len() + self.meta.len() + OBJECT_OVERHEAD
    }
}

const OBJECT_OVERHEAD: usize = 256;

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

/// An entity as the world remembers it once it is cut into objects.
#[derive(Debug)]
struct Entity {
    /// The hash of its JSON without `updatedAt` and text: a change of those
    /// alone is no new body.
    shape: u64,
    text_key: Option<String>,
}

#[derive(Debug, Default)]
struct ThreadState {
    entities: HashMap<(String, String), Entity>,
    /// Ordinal and text key of each assistant message, for the tail.
    assistant: BTreeMap<u64, String>,
    /// The ordinal of the newest user message: the current turn starts
    /// there.
    turn_start: u64,
    /// The ordinal of the turn item behind each turn item body and text
    /// object, by object id.
    turn_ordinals: HashMap<u64, u64>,
    /// The newest snapshot's frame fields and rows, encoded.
    frame_fields: Map<String, Value>,
    frame_rows: Vec<Box<[u8]>>,
    /// Rows of history pages that come from other threads (a fork's
    /// inherited rows), oldest first; they go before the snapshot's rows.
    older_rows: Vec<Box<[u8]>>,
    /// Every page of the thread's history is here.
    history_complete: bool,
    /// Attachments its turn items show.
    images: BTreeSet<String>,
    /// When it was last used, on the world's clock.
    used: u64,
}

/// A listed thread.
#[derive(Debug, Clone)]
pub struct Listed {
    /// `active` or `archive`.
    pub location: String,
    pub updated_at: String,
}

#[derive(Debug, Default)]
pub struct World {
    objects: HashMap<u64, Obj>,
    by_thread: HashMap<String, BTreeSet<u64>>,
    /// Threads with content in the cache.
    threads: HashMap<String, ThreadState>,
    shell_threads: HashMap<String, Listed>,
    shell_projects: HashSet<String>,
    changed: BTreeSet<u64>,
    /// The order of things with no better one: later is newer.
    seq: u64,
    /// Attachments asked for or here, and the threads that show them.
    images: HashMap<String, BTreeSet<String>>,
    /// The attachment of each image object.
    image_ids: HashMap<u64, String>,
    wanted_images: Vec<(String, String)>,
    shell_loaded: bool,
    /// Bytes of all objects, as `Obj::cost` counts them.
    bytes: usize,
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

/// The id of a thread's frame: a phone that has it has the thread.
pub fn frame_id(thread: &str) -> u64 {
    object_id(&["frame", thread])
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

    /// The objects of a thread in the cache, its row in the list and its
    /// images among them.
    pub fn thread_ids(&self, thread: &str) -> impl Iterator<Item = u64> + '_ {
        let images = self
            .threads
            .get(thread)
            .into_iter()
            .flat_map(|s| s.images.iter())
            .map(|a| object_id(&["image", a]))
            .filter(|id| self.objects.contains_key(id));
        self.by_thread
            .get(thread)
            .into_iter()
            .flatten()
            .copied()
            .chain(images)
    }

    /// The threads that show an image object.
    pub fn image_threads(&self, id: u64) -> impl Iterator<Item = &String> {
        self.image_ids
            .get(&id)
            .and_then(|a| self.images.get(a))
            .into_iter()
            .flatten()
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

    /// Whether an object is a message item, or its text, of the thread's
    /// current turn: from its newest user message on.
    pub fn in_current_turn(&self, thread: &str, id: u64) -> bool {
        self.threads
            .get(thread)
            .and_then(|s| Some(*s.turn_ordinals.get(&id)? >= s.turn_start))
            .unwrap_or(false)
    }

    /// The listed threads.
    pub fn listed(&self) -> impl Iterator<Item = (&String, &Listed)> {
        self.shell_threads.iter()
    }

    pub fn listed_thread(&self, thread: &str) -> Option<&Listed> {
        self.shell_threads.get(thread)
    }

    /// Whether the thread list is here.
    pub fn shell_loaded(&self) -> bool {
        self.shell_loaded
    }

    /// Whether the config, descriptor and session are here.
    pub fn config_loaded(&self) -> bool {
        [CONFIG, DESCRIPTOR, SESSION]
            .iter()
            .all(|c| self.objects.contains_key(&object_id(&[c])))
    }

    /// Whether the thread has content in the cache.
    pub fn resident(&self, thread: &str) -> bool {
        self.threads.contains_key(thread)
    }

    /// Whether every page of the thread's history is in the cache.
    pub fn history_complete(&self, thread: &str) -> bool {
        self.threads.get(thread).is_some_and(|s| s.history_complete)
    }

    /// Bytes of all objects in the world.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// Marks a thread as just used, so it is evicted last.
    pub fn touch(&mut self, thread: &str) {
        self.seq += 1;
        let seq = self.seq;
        self.threads.entry(thread.to_owned()).or_default().used = seq;
    }

    fn next_seq(&mut self) -> u64 {
        self.seq += 1;
        self.seq
    }

    /// The thread whose bytes an object counts to.
    fn owner(obj: &Obj) -> Option<&str> {
        match obj.kind {
            Kind::Body | Kind::Text | Kind::Frame => obj.thread.as_deref(),
            _ => None,
        }
    }

    fn count(&mut self, obj: &Obj, add: bool) {
        let cost = obj.cost();
        if add {
            self.bytes += cost;
        } else {
            self.bytes = self.bytes.saturating_sub(cost);
        }
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
        if let Some(t) = Self::owner(&obj) {
            self.threads.entry(t.to_owned()).or_default();
        }
        self.count(&obj, true);
        if let Some(old) = self.objects.insert(id, obj) {
            self.count(&old, false);
        }
        self.changed.insert(id);
    }

    /// Empties an object: the phone deletes what it had.
    fn delete(&mut self, id: u64) {
        let Some(obj) = self.objects.get(&id) else {
            return;
        };
        if obj.content.is_empty() {
            return;
        }
        let emptied = Obj {
            content: Arc::from([]),
            ..obj.clone()
        };
        self.put(id, emptied);
    }

    /// Evicts the threads used least recently, never one `keep` names,
    /// until the world holds at most `limit` bytes. Returns each evicted
    /// thread with the ids of its objects that went: phones keep their
    /// copies, the sidecar fetches the thread again when it is needed.
    pub fn evict_over(
        &mut self,
        limit: usize,
        keep: impl Fn(&str) -> bool,
    ) -> Vec<(String, Vec<u64>)> {
        let mut gone = Vec::new();
        while self.bytes > limit {
            let Some(t) = self
                .threads
                .iter()
                .filter(|(t, _)| !keep(t))
                .min_by_key(|(_, s)| s.used)
                .map(|(t, _)| t.clone())
            else {
                break;
            };
            let ids = self.evict(&t);
            gone.push((t, ids));
        }
        gone
    }

    /// Drops a thread's content and the images no other thread in the
    /// cache shows. Its row in the thread list stays.
    fn evict(&mut self, thread: &str) -> Vec<u64> {
        let mut gone = Vec::new();
        let Some(state) = self.threads.remove(thread) else {
            return gone;
        };
        let ids = self.by_thread.remove(thread).unwrap_or_default();
        let objects = &self.objects;
        let (rows, ids): (BTreeSet<u64>, BTreeSet<u64>) = ids
            .into_iter()
            .partition(|id| objects.get(id).is_some_and(|o| o.kind == Kind::Shell));
        if !rows.is_empty() {
            self.by_thread.insert(thread.to_owned(), rows);
        }
        for id in ids {
            if let Some(o) = self.objects.remove(&id) {
                self.bytes = self.bytes.saturating_sub(o.cost());
                self.changed.remove(&id);
                gone.push(id);
            }
        }
        for a in &state.images {
            let Some(threads) = self.images.get_mut(a) else {
                continue;
            };
            threads.remove(thread);
            if !threads.is_empty() {
                continue;
            }
            self.images.remove(a);
            let id = object_id(&["image", a]);
            self.image_ids.remove(&id);
            if let Some(o) = self.objects.remove(&id) {
                self.bytes = self.bytes.saturating_sub(o.cost());
                self.changed.remove(&id);
                gone.push(id);
            }
        }
        gone
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
                content: content.into(),
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
                content: compact(&header).into(),
            },
        );
        let mut threads = HashMap::new();
        for (location, key) in [("active", "threads"), ("archive", "archivedThreads")] {
            for t in snap[key].as_array().into_iter().flatten() {
                if let Some(id) = t["id"].as_str() {
                    threads.insert(id.to_owned(), (location, t));
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
            self.put_shell_thread(&id, location, t);
        }
        let projects: HashMap<String, &Value> = snap["projects"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|p| Some((p["id"].as_str()?.to_owned(), p)))
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
            self.put_project(&id, p);
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
                meta: compact(&json!({"c": "project", "i": hash_id(id)})),
                content: compact(project).into(),
            },
        );
    }

    fn put_shell_thread(&mut self, id: &str, location: &str, thread: &Value) {
        if let Some(old) = self.shell_threads.get(id)
            && old.location != location
        {
            let old = object_id(&["thread", &old.location, id]);
            self.delete(old);
        }
        self.shell_threads.insert(
            id.to_owned(),
            Listed {
                location: location.to_owned(),
                updated_at: thread["updatedAt"].as_str().unwrap_or_default().to_owned(),
            },
        );
        let order = self.next_seq();
        self.put(
            object_id(&["thread", location, id]),
            Obj {
                // The thread's own: its row goes with its status while it is open.
                thread: Some(id.to_owned()),
                kind: Kind::Shell,
                status: false,
                order,
                meta: compact(&json!({"c": "thread", "i": hash_id(id), "l": location})),
                content: compact(thread).into(),
            },
        );
    }

    fn remove_shell_thread(&mut self, id: &str) {
        if let Some(listed) = self.shell_threads.remove(id) {
            self.delete(object_id(&["thread", &listed.location, id]));
        }
        // Phones delete its content by the thread list (`App::prune`).
        self.evict(id);
    }

    // A thread's content.

    /// A thread's snapshot: a `subscribeThread` snapshot item or t3's
    /// bounded HTTP snapshot. When it holds the whole history, entities it
    /// does not have any more are deleted; a bounded window keeps the older
    /// ones.
    pub fn set_thread(&mut self, thread: &str, snapshot: &Value) {
        let projection = &snapshot["projection"];
        let Some(fields) = projection.as_object() else {
            return;
        };
        self.touch(thread);
        // A window with none of the items the world has leaves a gap
        // between them: the pages are fetched again (`App::thread_snapshot`).
        let overlaps = self.threads.get(thread).is_some_and(|s| {
            fields["turnItems"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|i| {
                    i["id"].as_str().is_some_and(|id| {
                        s.entities
                            .contains_key(&("turnItems".to_owned(), id.to_owned()))
                    })
                })
        });
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
        let complete = snapshot["hasMoreHistory"].as_bool() != Some(true);
        if complete {
            let gone: Vec<(String, String)> = self
                .threads
                .get(thread)
                .map(|t| {
                    t.entities
                        .keys()
                        .filter(|k| !seen.contains(*k))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();
            for (coll, id) in gone {
                self.remove_entity(thread, &coll, &id);
            }
            let state = self.threads.entry(thread.to_owned()).or_default();
            state.history_complete = true;
            state.older_rows.clear();
        } else if !overlaps {
            let state = self.threads.entry(thread.to_owned()).or_default();
            state.history_complete = false;
            state.older_rows.clear();
        }
        self.set_frame(thread, snapshot);
    }

    /// A page of `GET /api/orchestration/threads/:id/history`, older than
    /// what the world has of the thread. Returns whether more pages follow.
    pub fn add_history(&mut self, thread: &str, page: &Value) -> bool {
        let mut inherited: Vec<Box<[u8]>> = Vec::new();
        for row in page["items"].as_array().into_iter().flatten() {
            if row["visibility"].as_str() == Some("local") {
                self.put_entity(thread, "turnItems", &row["item"]);
            } else {
                inherited.push(compact(row).into());
            }
        }
        let more = page["hasMoreHistory"].as_bool() == Some(true);
        let state = self.threads.entry(thread.to_owned()).or_default();
        state.history_complete = !more;
        if !inherited.is_empty() {
            inherited.append(&mut state.older_rows);
            state.older_rows = inherited;
            self.put_frame(thread);
        }
        more
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
        let shape = shape_of(&body);
        let state = self.threads.entry(thread.to_owned()).or_default();
        let entity_key = (coll.to_owned(), id.clone());
        let unchanged = state
            .entities
            .get(&entity_key)
            .is_some_and(|old| old.shape == shape);
        state.entities.insert(
            entity_key,
            Entity {
                shape,
                text_key: key.clone(),
            },
        );
        if let (Some(key), Some(text)) = (&key, text) {
            if coll == "turnItems" {
                self.note_assistant(thread, entity, key);
                self.note_turn(thread, entity, object_id(&["text", thread, key]));
            }
            self.put_text(thread, key, &text_kind(coll, entity), text);
        }
        if coll == "turnItems" {
            self.note_turn(thread, entity, object_id(&["body", thread, coll, &id]));
        }
        let body_id = object_id(&["body", thread, coll, &id]);
        if unchanged && self.objects.contains_key(&body_id) {
            return;
        }
        let order = match entity["ordinal"].as_u64() {
            Some(o) if coll == "turnItems" => o,
            // An approval or question the owner must answer goes first in
            // the status class, and its answer too, so the phone stops
            // offering it. With the next sequence number it went after every
            // item of the current turn: on profile D the approval showed
            // 103 s after it was asked (issue #46).
            _ if coll == "runtimeRequests"
                && (entity["status"].as_str() == Some("pending")
                    || self
                        .objects
                        .get(&body_id)
                        .is_some_and(|o| o.order == u64::MAX)) =>
            {
                u64::MAX
            }
            _ => self.next_seq(),
        };
        let status = STATUS_COLLECTIONS.contains(&coll)
            || (coll == "turnItems"
                && matches!(
                    entity["status"].as_str(),
                    Some("pending" | "running" | "waiting")
                ));
        self.put(
            body_id,
            Obj {
                thread: Some(thread.to_owned()),
                kind: Kind::Body,
                status,
                order,
                meta: compact(&json!({"t": hash_id(thread), "c": coll, "i": hash_id(&id)})),
                content: compact(&body).into(),
            },
        );
        if coll == "turnItems" {
            self.want_images(thread, entity);
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
        let Some(key) = entity.text_key else {
            return;
        };
        let shown = state
            .entities
            .values()
            .any(|e| e.text_key.as_deref() == Some(key.as_str()));
        if shown {
            return;
        }
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
    /// class. Only the turn's messages count: tool rows can be large, and
    /// they would hold back the server config, without which the app shows
    /// nothing after a cache loss.
    fn note_turn(&mut self, thread: &str, item: &Value, id: u64) {
        let Some(ordinal) = item["ordinal"].as_u64() else {
            return;
        };
        if !matches!(
            item["type"].as_str(),
            Some("user_message" | "assistant_message")
        ) {
            return;
        }
        let state = self.threads.entry(thread.to_owned()).or_default();
        state.turn_ordinals.insert(id, ordinal);
        if item["type"].as_str() == Some("user_message") && ordinal > state.turn_start {
            state.turn_start = ordinal;
            self.changed.extend(state.turn_ordinals.keys().copied());
        }
    }

    fn put_text(&mut self, thread: &str, key: &str, kind: &str, text: String) {
        let id = object_id(&["text", thread, key]);
        let order = self
            .threads
            .get(thread)
            .and_then(|s| s.assistant.iter().find(|(_, k)| *k == key))
            .map(|(o, _)| *o);
        let order = match order {
            Some(o) => o,
            None => self.next_seq(),
        };
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
                meta: compact(&json!({"t": hash_id(thread), "m": hash_id(key), "k": kind})),
                content: text.into_bytes().into(),
            },
        );
    }

    fn set_frame(&mut self, thread: &str, snapshot: &Value) {
        let rows: Vec<Box<[u8]>> = snapshot["projection"]["visibleTurnItems"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|row| {
                if row["visibility"].as_str() == Some("local")
                    && let Some(obj) = row.as_object()
                {
                    let mut obj = obj.clone();
                    obj.remove("item");
                    return compact(&Value::Object(obj)).into();
                }
                compact(row).into()
            })
            .collect();
        let mut fields = Map::new();
        for key in [
            "snapshotSequence",
            "historyCursor",
            "hasMoreHistory",
            "latestLocalTurnOrdinal",
        ] {
            if let Some(v) = snapshot.get(key) {
                fields.insert(key.into(), v.clone());
            }
        }
        fields.insert(
            "updatedAt".into(),
            snapshot["projection"]["updatedAt"].clone(),
        );
        let state = self.threads.entry(thread.to_owned()).or_default();
        state.frame_fields = fields;
        state.frame_rows = rows;
        self.put_frame(thread);
    }

    /// The frame: the snapshot's fields, then `visibleTurnItems`, the
    /// older inherited rows before the snapshot's rows.
    fn put_frame(&mut self, thread: &str) {
        let Some(state) = self.threads.get(thread) else {
            return;
        };
        let mut content = compact(&Value::Object(state.frame_fields.clone()));
        // `{...}` becomes `{...,"visibleTurnItems":[...]}`.
        content.pop();
        if content.len() > 1 {
            content.push(b',');
        }
        content.extend_from_slice(br#""visibleTurnItems":["#);
        for (i, row) in state.older_rows.iter().chain(&state.frame_rows).enumerate() {
            if i > 0 {
                content.push(b',');
            }
            content.extend_from_slice(row);
        }
        content.extend_from_slice(b"]}");
        self.put(
            frame_id(thread),
            Obj {
                thread: Some(thread.to_owned()),
                kind: Kind::Frame,
                status: true,
                order: u64::MAX,
                meta: compact(&json!({"t": hash_id(thread), "c": "frame"})),
                content: content.into(),
            },
        );
    }

    // Images.

    fn want_images(&mut self, thread: &str, item: &Value) {
        for a in item["attachments"].as_array().into_iter().flatten() {
            if a["type"].as_str() != Some("image") {
                continue;
            }
            let (Some(id), Some(mime)) = (a["id"].as_str(), a["mimeType"].as_str()) else {
                continue;
            };
            self.threads
                .entry(thread.to_owned())
                .or_default()
                .images
                .insert(id.to_owned());
            let threads = self.images.entry(id.to_owned()).or_default();
            let new = threads.is_empty();
            threads.insert(thread.to_owned());
            if new {
                self.image_ids
                    .insert(object_id(&["image", id]), id.to_owned());
                self.wanted_images.push((id.to_owned(), mime.to_owned()));
            }
        }
    }

    /// An image's bytes. One no thread in the cache shows any more is
    /// dropped.
    pub fn set_image(&mut self, id: &str, mime: &str, bytes: Vec<u8>) {
        let Some(threads) = self.images.get(id) else {
            return;
        };
        // Phones delete images of threads that left the list by this.
        let owner = threads.iter().next().map(|t| hash_id(t));
        let order = self.next_seq();
        self.put(
            object_id(&["image", id]),
            Obj {
                thread: None,
                kind: Kind::Image,
                status: false,
                order,
                meta: compact(&json!({"c": "image", "i": hash_id(id), "y": mime, "o": owner})),
                content: bytes.into(),
            },
        );
    }

    /// Forgets that an image was asked for, so it is asked for again.
    pub fn image_failed(&mut self, id: &str) {
        self.images.remove(id);
        self.image_ids.remove(&object_id(&["image", id]));
    }
}

/// The hash of an entity's JSON without `updatedAt` and `text`.
fn shape_of(body: &Value) -> u64 {
    match body.as_object() {
        Some(o) => {
            let mut o = o.clone();
            o.remove("updatedAt");
            o.remove("text");
            hash(&compact(&Value::Object(o)))
        }
        None => hash(&compact(body)),
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
        w.get(id)
            .map(|o| String::from_utf8_lossy(&o.content).into_owned())
            .unwrap_or_default()
    }

    fn snap(thread: &str, items: Value, more: bool) -> Value {
        json!({"snapshotSequence": 3, "hasMoreHistory": more, "historyCursor": if more { json!("c1") } else { Value::Null },
            "projection": {"thread": {"id": thread}, "turnItems": items, "visibleTurnItems": [], "updatedAt": "now"}})
    }

    fn image_item(id: &str, ordinal: u64, attachment: &str) -> Value {
        json!({"id": id, "type": "user_message", "messageId": format!("m-{id}"), "ordinal": ordinal,
            "text": "look", "attachments": [{"type": "image", "id": attachment, "mimeType": "image/png"}]})
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
        let tool = json!({"id": "x", "threadId": "t1", "type": "command_execution", "ordinal": 5,
            "status": "completed", "updatedAt": "1"});
        w.apply_event("t1", &event("turn-item.updated", tool));
        assert!(
            !w.in_current_turn("t1", body("x")),
            "tool rows stay with history"
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
    fn a_pending_runtime_request_goes_first_in_its_class() {
        let mut w = World::default();
        let request = |status: &str| json!({"id": "rr1", "threadId": "t1", "status": status, "updatedAt": status});
        w.apply_event(
            "t1",
            &event("turn-item.updated", item("a", 9, "text", true, "1")),
        );
        w.apply_event("t1", &event("runtime-request.updated", request("pending")));
        let id = object_id(&["body", "t1", "runtimeRequests", "rr1"]);
        let turn_item = object_id(&["body", "t1", "turnItems", "a"]);
        let order = |w: &World, id| w.get(id).map(|o| o.order);
        // The approval the owner must answer, ahead of the turn's items, and
        // its answer too.
        assert!(order(&w, id) > order(&w, turn_item));
        w.apply_event("t1", &event("runtime-request.updated", request("resolved")));
        assert!(order(&w, id) > order(&w, turn_item));
        // One never pending keeps the next sequence number.
        w.apply_event(
            "t1",
            &event(
                "runtime-request.updated",
                json!({"id": "rr2", "threadId": "t1", "status": "expired"}),
            ),
        );
        let other = object_id(&["body", "t1", "runtimeRequests", "rr2"]);
        assert!(order(&w, other) < order(&w, turn_item));
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
    fn the_least_recently_used_thread_goes_with_the_images_only_it_shows() {
        let mut w = World::default();
        let big = "x".repeat(10_000);
        w.set_thread(
            "t1",
            &snap(
                "t1",
                json!([item("a", 1, &big, false, "1"), image_item("u1", 2, "only1")]),
                false,
            ),
        );
        w.set_thread(
            "t2",
            &snap(
                "t2",
                json!([
                    item("b", 1, &big, false, "1"),
                    image_item("u2", 2, "shared")
                ]),
                false,
            ),
        );
        w.set_thread(
            "t3",
            &snap(
                "t3",
                json!([
                    item("c", 1, &big, false, "1"),
                    image_item("u3", 2, "shared")
                ]),
                false,
            ),
        );
        w.take_wanted_images();
        for a in ["only1", "shared"] {
            w.set_image(a, "image/png", vec![7; 5_000]);
        }
        w.take_changed();
        // t1 is used again: t2 is the least recently used now.
        w.touch("t1");
        let before = w.bytes();
        let gone: Vec<u64> = w
            .evict_over(before - 1, |t| t == "t3")
            .into_iter()
            .flat_map(|(_, ids)| ids)
            .collect();
        assert!(gone.contains(&object_id(&["text", "t2", "m-b"])));
        assert!(!w.resident("t2") && w.resident("t1") && w.resident("t3"));
        // The shared image stays for t3; the thread's row in the list too.
        assert!(w.get(object_id(&["image", "shared"])).is_some());
        assert!(w.bytes() < before);
        // Next t1, and its own image with it; t3 is kept.
        let gone: Vec<u64> = w
            .evict_over(0, |t| t == "t3")
            .into_iter()
            .flat_map(|(_, ids)| ids)
            .collect();
        assert!(gone.contains(&object_id(&["image", "only1"])));
        assert!(!gone.contains(&object_id(&["image", "shared"])));
        assert!(w.resident("t3") && !w.resident("t1"));
        assert!(
            w.take_changed().is_empty(),
            "eviction changes nothing on phones"
        );
        // An evicted image is asked for again when a thread shows it.
        w.set_thread(
            "t1",
            &snap("t1", json!([image_item("u1", 2, "only1")]), false),
        );
        assert_eq!(
            w.take_wanted_images(),
            vec![("only1".to_owned(), "image/png".to_owned())]
        );
    }

    #[test]
    fn a_thread_loaded_again_after_eviction_is_the_same_objects() {
        let mut w = World::default();
        let s = snap(
            "t1",
            json!([
                item("a", 1, "first", false, "1"),
                item("b", 2, "second", false, "1")
            ]),
            false,
        );
        w.set_thread("t1", &s);
        let before: BTreeMap<u64, Vec<u8>> = w
            .thread_ids("t1")
            .map(|id| (id, w.get(id).unwrap().content.to_vec()))
            .collect();
        w.evict_over(0, |_| false);
        assert_eq!(w.thread_ids("t1").count(), 0);
        w.set_thread("t1", &s);
        let after: BTreeMap<u64, Vec<u8>> = w
            .thread_ids("t1")
            .map(|id| (id, w.get(id).unwrap().content.to_vec()))
            .collect();
        assert_eq!(before, after);
    }

    #[test]
    fn history_pages_add_older_items_and_inherited_rows() {
        let mut w = World::default();
        w.set_thread(
            "t1",
            &snap("t1", json!([item("c", 3, "newest", false, "1")]), true),
        );
        assert!(!w.history_complete("t1"));
        let page = |items: Value, more: bool| json!({"snapshotSequence": 3, "items": items, "nextCursor": if more { json!("c2") } else { Value::Null }, "hasMoreHistory": more});
        let local = |i: Value| json!({"position": 0, "visibility": "local", "sourceThreadId": "t1", "sourceItemId": i["id"], "item": i});
        assert!(w.add_history(
            "t1",
            &page(json!([local(item("b", 2, "older", false, "1"))]), true)
        ));
        let inherited = json!({"position": 0, "visibility": "inherited", "sourceThreadId": "p", "sourceItemId": "x", "item": {"id": "x"}});
        assert!(!w.add_history(
            "t1",
            &page(
                json!([inherited, local(item("a", 1, "oldest", false, "1"))]),
                false
            )
        ));
        assert!(w.history_complete("t1"));
        for (id, text) in [("a", "oldest"), ("b", "older"), ("c", "newest")] {
            assert_eq!(
                content(&w, object_id(&["text", "t1", &format!("m-{id}")])),
                text
            );
        }
        let frame: Value = serde_json::from_str(&content(&w, frame_id("t1"))).unwrap();
        assert_eq!(frame["visibleTurnItems"][0]["sourceItemId"], "x");
        assert_eq!(frame["hasMoreHistory"], true);
        // A newer window that reaches back to what the world has keeps it all.
        let newer = json!([
            item("c", 3, "newest", false, "1"),
            item("d", 4, "newer", false, "1")
        ]);
        w.set_thread("t1", &snap("t1", newer, true));
        assert_eq!(content(&w, object_id(&["text", "t1", "m-a"])), "oldest");
        assert!(w.history_complete("t1"));
        // One that does not leaves a gap: its pages are fetched again.
        let far = json!([item("z", 99, "much newer", false, "1")]);
        w.set_thread("t1", &snap("t1", far, true));
        assert!(!w.history_complete("t1"));
    }

    #[test]
    fn long_ids_are_hashed_in_meta_and_nothing_is_left_out() {
        let mut w = World::default();
        // Twice the longest id in the main history.
        let thread = "thread:delegated-task:".to_owned() + &"%2525".repeat(280);
        let message = "message:".to_owned() + &"%25".repeat(470);
        assert!(thread.len() >= 1_400 && message.len() >= 1_400);
        w.set_shell(
            &json!({"kind": "snapshot", "snapshot": {"schemaVersion": 2, "snapshotSequence": 1,
            "projects": [{"id": thread}], "threads": [{"id": thread}], "archivedThreads": []}}),
        );
        let mut it = item("a", 1, "text", false, "1");
        it["id"] = json!(message);
        it["messageId"] = json!(message);
        w.set_thread(
            &thread,
            &snap(&thread, json!([it, image_item("u", 2, &message)]), false),
        );
        w.set_image(&message, "image/png", vec![1, 2, 3]);
        assert!(w.skipped.is_empty(), "{:?}", w.skipped);
        let text = w.get(object_id(&["text", &thread, &message])).unwrap();
        let meta: Value = serde_json::from_slice(&text.meta).unwrap();
        assert_eq!(meta["t"], hash_id(&thread));
        assert_eq!(meta["m"], hash_id(&message));
        let image = w.get(object_id(&["image", &message])).unwrap();
        assert_eq!(
            serde_json::from_slice::<Value>(&image.meta).unwrap()["i"],
            hash_id(&message)
        );
        for id in w.ids() {
            assert!(
                w.get(id).unwrap().meta.len() <= 100,
                "{}",
                String::from_utf8_lossy(&w.get(id).unwrap().meta)
            );
        }
    }
}
