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
    Command::new(&cmd.bin)
        .args(&cmd.args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())
}

pub fn run_all(cmds: Vec<Cmd>) -> Result<Vec<String>, String> {
    cmds.into_iter()
        .map(|c| spawn(&c))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|c| {
            let o = c.wait_with_output().map_err(|e| e.to_string())?;
            if o.status.success() {
                Ok(String::from_utf8_lossy(&o.stdout).into_owned())
            } else {
                Err(format!(
                    "command failed: {}",
                    String::from_utf8_lossy(&o.stderr)
                ))
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(bin: &'static str, args: Vec<String>) -> Result<Vec<String>, String> {
        let cmd = Cmd::new(bin, args);
        run_all(vec![cmd])
    }

    #[test]
    fn run_non_existing() {
        let cmd = Cmd {
            bin: "i-do-not-exist",
            args: vec![],
        };
        let res = run_all(vec![cmd]);
        assert!(res.is_err());
        assert_eq!(
            res.err(),
            Some("No such file or directory (os error 2)".to_owned())
        );
    }

    #[test]
    fn run_exit_code() {
        let cmd = Cmd::new("sh", ["-c", "exit 1"]);
        let res = run_all(vec![cmd]);
        assert!(res.is_err());
        assert_eq!(res.err(), Some("command failed: ".to_owned()));
    }

    #[test]
    fn run_valid() {
        let cmd = Cmd {
            bin: "cargo",
            args: vec![],
        };
        let res = run_all(vec![cmd]);
        assert!(res.is_ok());
        assert!(!res.unwrap()[0].is_empty())
    }
}
