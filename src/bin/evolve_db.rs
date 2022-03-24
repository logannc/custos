use evolve_derive::list_evolves;

use custos::models::foo_model::Foo;

fn main() {
    let f = Foo{};
    list_evolves!();
    println!("test test");
}

