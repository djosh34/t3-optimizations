//! The sidecar's settings, from flags or one TOML file with the same names
//! (`--t3-url` is `t3-url = "..."`). Flags win over the file. Nothing has a
//! built-in port or path.

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

use serde::Deserialize;

pub const USAGE: &str = "usage: sidecar [--config <file.toml>] [flags]

  --t3-url <url>          the t3 server, such as http://127.0.0.1:13773
  --t3-base-dir <dir>     its --base-dir, which every t3 command gets
  --t3-command <cmd>      how to run its CLI, such as \"node /opt/t3opt/t3code/apps/server/src/bin.ts\"
  --data-dir <dir>        the sidecar's own data: tokens, certificate, phones
  --http <addr>           HTTP listen address, such as 127.0.0.1:12480
  --udp <addr>            UDP address of the data channel, such as 0.0.0.0:12444
  --udp-advertise <ip>    the address phones send UDP to, when --udp binds 0.0.0.0
  --web-dir <dir>         the web app's files (optional)
  --cache-mb <n>          the threads and images held in memory, in MB (default 256)
  --preload <n>           how many of the newest threads phones get ahead of time; 0 is off (default 50)";

/// Defaults of the settings that have one (issue #81).
const CACHE_MB: usize = 256;
const PRELOAD: usize = 50;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct File {
    t3_url: Option<String>,
    t3_base_dir: Option<PathBuf>,
    t3_command: Option<String>,
    data_dir: Option<PathBuf>,
    http: Option<SocketAddr>,
    udp: Option<SocketAddr>,
    udp_advertise: Option<IpAddr>,
    web_dir: Option<PathBuf>,
    cache_mb: Option<usize>,
    preload: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub t3_url: String,
    pub t3_base_dir: PathBuf,
    pub t3_command: Vec<String>,
    pub data_dir: PathBuf,
    pub http: SocketAddr,
    pub udp: SocketAddr,
    pub udp_advertise: Option<IpAddr>,
    pub web_dir: Option<PathBuf>,
    pub cache_mb: usize,
    pub preload: usize,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

impl Config {
    pub fn from_args(args: Vec<String>) -> Result<Config, Error> {
        let mut file = File::default();
        let mut flags = File::default();
        let mut it = args.into_iter();
        while let Some(flag) = it.next() {
            let value = it
                .next()
                .ok_or_else(|| format!("{flag} needs a value\n{USAGE}"))?;
            match flag.as_str() {
                "--config" => file = toml::from_str(&std::fs::read_to_string(&value)?)?,
                "--t3-url" => flags.t3_url = Some(value),
                "--t3-base-dir" => flags.t3_base_dir = Some(value.into()),
                "--t3-command" => flags.t3_command = Some(value),
                "--data-dir" => flags.data_dir = Some(value.into()),
                "--http" => flags.http = Some(value.parse()?),
                "--udp" => flags.udp = Some(value.parse()?),
                "--udp-advertise" => flags.udp_advertise = Some(value.parse()?),
                "--web-dir" => flags.web_dir = Some(value.into()),
                "--cache-mb" => flags.cache_mb = Some(value.parse()?),
                "--preload" => flags.preload = Some(value.parse()?),
                _ => return Err(format!("unknown flag {flag}\n{USAGE}").into()),
            }
        }
        let missing = |name: &str| format!("missing --{name}\n{USAGE}");
        let command: Vec<String> = flags
            .t3_command
            .or(file.t3_command)
            .ok_or_else(|| missing("t3-command"))?
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        if command.is_empty() {
            return Err(missing("t3-command").into());
        }
        Ok(Config {
            t3_url: flags
                .t3_url
                .or(file.t3_url)
                .ok_or_else(|| missing("t3-url"))?
                .trim_end_matches('/')
                .to_owned(),
            t3_base_dir: flags
                .t3_base_dir
                .or(file.t3_base_dir)
                .ok_or_else(|| missing("t3-base-dir"))?,
            t3_command: command,
            data_dir: flags
                .data_dir
                .or(file.data_dir)
                .ok_or_else(|| missing("data-dir"))?,
            http: flags.http.or(file.http).ok_or_else(|| missing("http"))?,
            udp: flags.udp.or(file.udp).ok_or_else(|| missing("udp"))?,
            udp_advertise: flags.udp_advertise.or(file.udp_advertise),
            web_dir: flags.web_dir.or(file.web_dir),
            cache_mb: flags.cache_mb.or(file.cache_mb).unwrap_or(CACHE_MB),
            preload: flags.preload.or(file.preload).unwrap_or(PRELOAD),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &[&str]) -> Vec<String> {
        s.iter().map(|a| a.to_string()).collect()
    }

    #[test]
    fn flags_win_over_the_file() -> Result<(), Error> {
        let path = std::env::temp_dir().join(format!("sidecar-config-{}.toml", std::process::id()));
        std::fs::write(
            &path,
            r#"
t3-url = "http://127.0.0.1:1/"
t3-base-dir = "/b"
t3-command = "node bin.ts"
data-dir = "/d"
http = "127.0.0.1:2"
udp = "0.0.0.0:3"
"#,
        )?;
        let c = Config::from_args(args(&[
            "--config",
            path.to_str().unwrap_or_default(),
            "--udp",
            "0.0.0.0:4",
        ]))?;
        std::fs::remove_file(&path)?;
        assert_eq!(c.t3_url, "http://127.0.0.1:1");
        assert_eq!(c.t3_command, ["node", "bin.ts"]);
        assert_eq!(c.udp.port(), 4);
        assert_eq!(c.web_dir, None);
        assert_eq!((c.cache_mb, c.preload), (CACHE_MB, PRELOAD));
        Ok(())
    }

    #[test]
    fn every_required_setting_is_named() {
        let e = Config::from_args(args(&["--t3-command", "t3"]))
            .err()
            .map(|e| e.to_string());
        assert!(e.is_some_and(|e| e.starts_with("missing --t3-url")));
    }
}
