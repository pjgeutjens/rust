use modules::greet; // so we can use greet without module specification
use rand::{thread_rng, Rng}; // use a function from a crate (see Cargo.toml)
fn main() {
    // modules::greet();
    greet();
    let x = thread_rng().gen_range(0, 100);
}
