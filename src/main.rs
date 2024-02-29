mod program;
mod interact;
mod example_programs;
pub mod example_interact_programs;
mod simple_env;


fn main() {
    let _: simple_env::Env = interact::interact(&example_interact_programs::hello_world());
}
