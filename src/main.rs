mod program;
mod interact;
mod example_programs;
pub mod example_interact_programs;
mod simple_env;

use crate::interact::Environment;


fn main() {
    interact::interact(&mut simple_env::Env::make(),&example_interact_programs::hello_world());
}
