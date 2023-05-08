//! Exec command
use super::Command;
use crate::Error;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

/// Abstract Exec Command
///
/// ```sh
/// leetcode-exec
/// Submit solution
///
/// USAGE:
///     leetcode exec <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```
pub struct ExecCommand;

#[async_trait]
impl Command for ExecCommand {
    /// `exec` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("exec")
            .about("Submit solution")
            .visible_alias("x")
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                    .help("question id"),
            )
            .arg(
                Arg::new("file")
                .short('f')
                .long("file")
                .num_args(1)
                .required(false)
                .help("custom file to submit")
                )
            .arg(
                Arg::new("lang")
                .short('l')
                .long("lang")
                .num_args(1)
                .required(false)
                .help("custom language to set")
                )
    }

    /// `exec` handler
    async fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let id: i32 = *m.get_one::<i32>("id").ok_or(Error::NoneError)?;
        let file_path = m.get_one::<String>("file").cloned();
        let lang = m.get_one::<String>("lang").cloned();
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit, None, file_path, lang).await?;

        println!("{}", res);
        Ok(())
    }
}
