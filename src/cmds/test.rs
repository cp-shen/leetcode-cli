//! Test command
use std::path::Path;
use crate::helper::get_meta_from_file;
use super::Command;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

/// Abstract Test Command
///
/// ```sh
/// leetcode-test
/// Edit question by id
///
/// USAGE:
///     leetcode test <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```
pub struct TestCommand;

#[async_trait]
impl Command for TestCommand {
    /// `test` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("test")
            .about("Test question by id")
            .visible_alias("t")
            .arg(
                Arg::new("file")
                // .short('f')
                // .long("file")
                .num_args(1)
                .required(true)
                .help("custom file to submit")
                )
            // .arg(
            //     Arg::new("id")
            //         .num_args(1)
            //         .required(true)
            //         .value_parser(clap::value_parser!(i32))
            //         .help("question id"),
            // )
            // .arg(
            //     Arg::new("testcase")
            //         .num_args(1)
            //         .required(false)
            //         .help("custom testcase"),
            // )
    }

    /// `test` handler
    async fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let file_path = m.get_one::<String>("file").unwrap().clone();
        let (id, lang) = get_meta_from_file(Path::new(&file_path)).unwrap();

        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Test, None, Some(file_path), Some(lang)).await?;

        println!("{}", res);
        Ok(())
    }
}
