mod memory;

mod instructions;
use instructions::{compile, assemble};

mod binary;
use binary::{link, run};

fn main() {
    let code = std::fs::read_to_string("test.g")
        .expect("Failed to read test.g");

    println!("code\n{}", code);

    let bin = link(
        assemble(
            compile(code)
        )
    );
    println!("bin {}", bin.to_string());
    
    run(bin);
}
