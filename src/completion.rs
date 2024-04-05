use clap::Command;
use clap_complete::{generate, Generator, Shell};

#[derive(clap::Args, Debug)]
pub struct CompletionArgs {
    #[arg(required = true)]
    generator: Option<Shell>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

impl CompletionArgs {
    pub fn run(&self, cmd: &mut Command) {
        print_completions(self.generator.unwrap(), cmd)
    }
}
