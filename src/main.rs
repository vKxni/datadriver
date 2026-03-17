mod csv;
mod cli;
mod entry;
mod explorer;
mod scanner;
mod utils;
mod writer;
mod datadriver;

fn main() {
    datadriver::run();
}
