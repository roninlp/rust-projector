use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expected to exist");
        if term == "add" {
            if value.len() != 3 {
                let err = anyhow!("add expects 2 arguments but got {}", value.len() - 1);
                return Err(err);
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }

        if term == "remove" {
            if value.len() != 2 {
                let err = anyhow!("remove expects 2 arguments but got {}", value.len() - 1);
                return Err(err);
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }
        if value.len() > 1 {
            let err = anyhow!("print expects 0 or 1 arguments but got {}", value.len());
            return Err(err);
        }
        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let loc = std::env::var("HOME").context("unable to get HOME")?;
    let mut loc = PathBuf::from(loc);

    loc.push(".config");
    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("errored getting current directory")?);
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{config::Operation, opts::Opts};

    use super::Config;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));

        Ok(())
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("foo")],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some(String::from("foo"))));

        Ok(())
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let opts: Config = Opts {
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(
            opts.operation,
            Operation::Add(String::from("foo"), String::from("bar"))
        );

        Ok(())
    }

    #[test]
    fn test_remove_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("remove"), String::from("foo")],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove(String::from("foo")));

        Ok(())
    }
}
