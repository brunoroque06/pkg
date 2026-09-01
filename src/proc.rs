use std::process::{Child, Command, Stdio};

pub struct Cmd {
    pub bin: &'static str,
    pub args: Vec<String>,
}

impl Cmd {
    pub fn new(bin: &'static str, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            bin,
            args: args.into_iter().map(Into::into).collect(),
        }
    }
}

fn spawn(cmd: &Cmd) -> Result<Child, String> {
    Command::new(cmd.bin)
        .args(&cmd.args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())
}

const CMD_FAILED: &str = "command failed: ";

pub fn run_all(cmds: Vec<Cmd>, chunk_size: usize) -> Result<Vec<String>, String> {
    let mut outs = Vec::with_capacity(cmds.len());

    for chunk in cmds.chunks(chunk_size) {
        let childs = chunk.iter().map(spawn).collect::<Result<Vec<_>, _>>()?;

        for c in childs {
            let o = c.wait_with_output().map_err(|e| e.to_string())?;
            if o.status.success() {
                outs.push(String::from_utf8_lossy(&o.stdout).into_owned());
            } else {
                return Err(format!(
                    "{}{}",
                    CMD_FAILED,
                    String::from_utf8_lossy(&o.stderr)
                ));
            }
        }
    }

    Ok(outs)
}

#[cfg(test)]
mod test {
    use super::*;

    const CHUNK_SIZE: usize = 2;

    #[test]
    fn run_non_existing() {
        let cmd = Cmd {
            bin: "i-do-not-exist",
            args: vec![],
        };
        let res = run_all(vec![cmd], CHUNK_SIZE);
        assert!(res.is_err());
        assert_eq!(
            res.err(),
            Some("No such file or directory (os error 2)".to_owned())
        );
    }

    #[test]
    fn run_exit_code() {
        let cmd = Cmd::new("sh", ["-c", "exit 1"]);
        let res = run_all(vec![cmd], CHUNK_SIZE);
        assert!(res.is_err());
        assert_eq!(res.err(), Some(CMD_FAILED.to_owned()));
    }

    #[test]
    fn run_prints_err() {
        let cmd = Cmd::new("cargo", ["wrong"]);
        let res = run_all(vec![cmd], CHUNK_SIZE);
        assert!(res.is_err());
        assert_ne!(res.err(), Some(CMD_FAILED.to_owned()));
    }

    #[test]
    fn run_valid() {
        let cmd = Cmd {
            bin: "cargo",
            args: vec![],
        };
        let res = run_all(vec![cmd], CHUNK_SIZE);
        assert!(res.is_ok());
        assert!(!res.unwrap()[0].is_empty())
    }
}
