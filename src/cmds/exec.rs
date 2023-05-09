//! Exec command
use super::Command;
use crate::helper::get_meta_from_file;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use std::path::Path;

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
            // .arg(
            //     Arg::new("id")
            //         .num_args(1)
            //         .required(true)
            //         .value_parser(clap::value_parser!(i32))
            //         .help("question id"),
            // )
            .arg(
                Arg::new("file")
                    // .short('f')
                    // .long("file")
                    .num_args(1)
                    .required(true)
                    .help("custom file to submit"),
            )
        // .arg(
        //     Arg::new("lang")
        //     .short('l')
        //     .long("lang")
        //     .num_args(1)
        //     .required(false)
        //     .help("custom language to set")
        //     )
    }

    /// `exec` handler
    async fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let file_path = m.get_one::<String>("file").unwrap().clone();
        let (id, lang) = get_meta_from_file(Path::new(&file_path)).unwrap();

        let cache = Cache::new()?;
        let res = cache
            .exec_problem(id, Run::Submit, None, Some(file_path), Some(lang))
            .await?;

        println!("{}", res);
        Ok(())
    }
}
