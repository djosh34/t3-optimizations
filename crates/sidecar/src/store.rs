//! The sidecar's data dir: its t3 bearer token, its DTLS certificate, and
//! one record and one sync state per paired phone.
//!
//! ```text
//! <data dir>/t3-token             bearer token for the forked t3 server
//! <data dir>/cert.der, key.der    DTLS certificate of every connection
//! <data dir>/devices/<id>.json    a paired phone: token, certificate, generation
//! <data dir>/devices/<id>.sync    its sync-core Server state (Server::save)
//! <data dir>/link-reports.jsonl   link reports the web app posts
//! ```
//!
//! Every file is written whole to a temp file and renamed, readable by the
//! owner only.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use pipe::generate_cert;
use serde::{Deserialize, Serialize};
use str0m::config::DtlsCert;

/// Writes a file only its owner can read, atomically.
pub fn write_private(path: &Path, data: &[u8]) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))?;
    }
    let tmp = path.with_extension("tmp");
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&tmp)?;
    f.write_all(data)?;
    f.sync_all()?;
    fs::rename(&tmp, path)
}

/// Appends a line to a file, starting it over past `max` bytes.
pub fn append_line(path: &Path, line: &[u8], max: u64) -> io::Result<()> {
    if fs::metadata(path).is_ok_and(|m| m.len() > max) {
        fs::rename(path, path.with_extension("jsonl.old"))?;
    }
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .mode(0o600)
        .open(path)?;
    f.write_all(line)?;
    f.write_all(b"\n")
}

/// A paired phone, as saved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub id: String,
    /// The 128-bit token, hex.
    pub token: String,
    /// The phone's DTLS certificate fingerprint from its last offer, as SDP
    /// writes it.
    pub fingerprint: Option<String>,
    /// The newest connection generation seen.
    pub generation: u64,
}

pub struct DataDir {
    root: PathBuf,
}

impl DataDir {
    pub fn new(root: PathBuf) -> io::Result<DataDir> {
        fs::create_dir_all(root.join("devices"))?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        Ok(DataDir { root })
    }

    pub fn token_file(&self) -> PathBuf {
        self.root.join("t3-token")
    }

    pub fn link_reports(&self) -> PathBuf {
        self.root.join("link-reports.jsonl")
    }

    /// The certificate of every connection, made on first use.
    pub fn cert(&self) -> Result<DtlsCert, Box<dyn std::error::Error + Send + Sync>> {
        let (cert, key) = (self.root.join("cert.der"), self.root.join("key.der"));
        if let (Ok(certificate), Ok(private_key)) = (fs::read(&cert), fs::read(&key)) {
            return Ok(DtlsCert {
                certificate,
                private_key,
            });
        }
        let made = generate_cert()?;
        write_private(&key, &made.private_key)?;
        write_private(&cert, &made.certificate)?;
        Ok(made)
    }

    fn device_path(&self, id: &str, ext: &str) -> PathBuf {
        self.root.join("devices").join(format!("{id}.{ext}"))
    }

    pub fn devices(&self) -> io::Result<Vec<DeviceRecord>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(self.root.join("devices"))? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "json") {
                let data = fs::read(&path)?;
                let record: DeviceRecord = serde_json::from_slice(&data)
                    .map_err(|e| io::Error::other(format!("{}: {e}", path.display())))?;
                out.push(record);
            }
        }
        Ok(out)
    }

    pub fn save_device(&self, record: &DeviceRecord) -> io::Result<()> {
        let data = serde_json::to_vec_pretty(record).map_err(io::Error::other)?;
        write_private(&self.device_path(&record.id, "json"), &data)
    }

    pub fn load_sync(&self, id: &str) -> io::Result<Option<Vec<u8>>> {
        match fs::read(self.device_path(id, "sync")) {
            Ok(d) => Ok(Some(d)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn save_sync(&self, id: &str, state: &[u8]) -> io::Result<()> {
        write_private(&self.device_path(id, "sync"), state)
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn parse_token(s: &str) -> Option<[u8; 16]> {
    let s = s.trim();
    if s.len() != 32 {
        return None;
    }
    let mut out = [0u8; 16];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    Some(out)
}

/// Random bytes from the OS.
pub fn random<const N: usize>() -> io::Result<[u8; N]> {
    let mut out = [0u8; N];
    getrandom::fill(&mut out).map_err(|e| io::Error::other(e.to_string()))?;
    Ok(out)
}

/// A new device id: 8 lowercase letters and digits.
pub fn new_device_id() -> io::Result<String> {
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    Ok(random::<8>()?
        .iter()
        .map(|b| ALPHABET[*b as usize % ALPHABET.len()] as char)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_hex_round_trip() {
        let t = [0xab; 16];
        assert_eq!(parse_token(&hex(&t)), Some(t));
        assert_eq!(parse_token("xyz"), None);
        assert_eq!(parse_token(&"g".repeat(32)), None);
    }

    #[test]
    fn device_ids_are_valid() -> io::Result<()> {
        for _ in 0..20 {
            assert!(pipe::creds::valid_device(&new_device_id()?));
        }
        Ok(())
    }
}
