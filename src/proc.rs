use std::process::{Child, Command, Stdio};

pub struct Cmd {
    pub bin: String,
    pub args: Vec<String>,
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

    fn run(bin: &str, args: Vec<String>) -> Result<Vec<String>, String> {
        let cmd = Cmd {
            bin: bin.to_owned(),
            args,
        };
        run_all(vec![cmd])
    }

    #[test]
    fn run_non_existing() {
        let res = run("i-dont-exist", vec![]);
        assert!(res.is_err());
        assert_eq!(
            res.err(),
            Some("No such file or directory (os error 2)".to_owned())
        );
    }

    #[test]
    fn run_exit_code() {
        let res = run("sh", vec!["-c".to_owned(), "exit 1".to_owned()]);
        assert!(res.is_err());
        assert_eq!(res.err(), Some("command failed: ".to_owned()));
    }

    #[test]
    fn run_valid() {
        let res = run("cargo", vec![]);
        assert!(res.is_ok());
        assert!(!res.unwrap()[0].is_empty())
    }
}
