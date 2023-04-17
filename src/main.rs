mod memory;

mod instructions;
use instructions::{compile, assemble};

mod binary;
use binary::{link};

fn main() {
    let code = "add 1 2\ngoto 3".to_owned();
    let bin = link(
        assemble(
            compile(code)
        )
    );
    println!("{}", bin.to_string());
}
