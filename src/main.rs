mod csv;
mod cli;
mod entry;
mod explorer;
mod scanner;
mod utils;
mod writer;
mod datadriver;
mod cli_commands;

fn main() {
    datadriver::run();
}
