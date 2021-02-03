use std::path::PathBuf;
use std::process;

use structopt::StructOpt;

mod github;

#[derive(Debug, StructOpt)]
#[structopt()]
enum Opt {
    /// process a benchmark file
    ProcessBenchmark {
        #[structopt(parse(from_os_str), help = "benchmark file")]
        input: PathBuf
    },

    /// test github
    TestGithub {
        #[structopt(short, long, env = "GITHUB_TOKEN")]
        token: String,

        #[structopt(short, long, env = "GITHUB_CONTEXT")]
        context: String
    }
}

#[tokio::main]
async fn main() {
    match Opt::from_args() {
        Opt::ProcessBenchmark { input } => {
            if !input.exists() || !input.is_file() {
                eprintln!("File {:?} could not be found", input);
                process::exit(1);
            }
            println!("{:?}", input);
        },
        Opt::TestGithub { token: _, context } => {
            github::test_github(&context).await;
        }
    }
}
