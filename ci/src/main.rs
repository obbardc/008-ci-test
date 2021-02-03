use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
enum Opt {
    #[structopt(help = "process a benchmark file")]
    ProcessBenchmark {
        #[structopt(parse(from_os_str), help = "benchmark file")]
        input: PathBuf,
    }
}

fn main() {
    match Opt::from_args() {
        Opt::ProcessBenchmark { input } => {
            if !input.exists() || !input.is_file() {
                eprintln!("File {:?} could not be found", input);
                process::exit(1);
            }
            println!("{:?}", input);
        }
    }
}
