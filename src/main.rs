mod memory;

mod instructions;
use instructions::{compile, assemble};

mod binary;
use binary::{link, run};

fn main() {
    let code = "add 1 2".to_owned();
    let bin = link(
        assemble(
            compile(code)
        )
    );
    println!("{}", bin.to_string());
    
    run(bin);
}
