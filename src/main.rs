mod cli;
mod cli_commands;
mod datadriver;
mod entry;
mod explorer;
mod scanner;
mod utils;
mod writer;

fn main() {
    datadriver::run();
}
