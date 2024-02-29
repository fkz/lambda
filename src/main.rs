pub mod example_interact_programs;
mod example_programs;
mod human_readable;
mod interact;
mod program;
mod simple_env;

use crate::interact::Environment;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let program = human_readable::parse_file(&path);
    interact::interact(&mut simple_env::Env::make(), &program);
}
