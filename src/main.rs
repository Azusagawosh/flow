mod cli;
mod projects;
mod notes;
mod store;
mod config;
mod idea;
mod terminal;
mod git;
mod github;

fn main() {
   store::init();
   cli::run();
}
