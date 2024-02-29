mod program;
mod interact;
mod example_programs;
pub mod example_interact_programs;
mod simple_env;
mod human_readable;

use crate::interact::Environment;


fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let program = human_readable::parse_file(&path);
    interact::interact(&mut simple_env::Env::make(), &program);
}
