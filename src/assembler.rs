use std::collections::HashMap;

enum SEGMENT {
    NotYet,
    Data,
    Code,
    Stack
}

pub fn compile_asm_to_bin(asem: String) -> Vec<i8> {
    let mut bin: Vec<i8> = vec![
        0, 0, 0 // goto Code segment (last 0 will be set to Code segment)
    ];
    
    let mut vars: HashMap<String, u8> = HashMap::new();
    let mut cseg = 0i8;
    let mut flag: SEGMENT = SEGMENT::NotYet;

    for line in asem.lines() {
        let tokens: Vec<String> = line.trim_start()
            .split(" ")
            .map(|s| {s.to_owned()})
            .collect();

        println!("{:?} {:?} {}", match flag {
            SEGMENT::Data => "data",
            SEGMENT::Code => "code",
            SEGMENT::NotYet => "NotYet",
            SEGMENT::Stack => "stack",
        }, bin, line);

        match flag {
            SEGMENT::NotYet => {
                match tokens[0].as_str() {
                    "//" => {},
                    "" => {},
                    "seg" => {
                        new_seg(&mut flag, &mut cseg, &tokens[1], bin.len() as i8);
                    },
                    _ => {}
                }
            },

            SEGMENT::Stack => {
                match tokens[0].as_str() {
                    "//" => {},
                    "" => {},
                    "seg" => {
                        new_seg(&mut flag, &mut cseg, &tokens[1], bin.len() as i8);
                    },
                    "byte" => {
                        // let count = tokens[1].parse::<u8>().unwrap_or(1);
                        // for _ in 0..count {
                            // bin.push(0);
                        // }
                    },
                    _ => {}
                }
            }

            SEGMENT::Data => {
                match tokens[0].as_str() {
                    "//" => {},
                    "" => {},
                    "seg" => {
                        new_seg(&mut flag, &mut cseg, &tokens[1], bin.len() as i8);
                    },
                    "byte" => {
                        bin.push( tokens[2].parse::<i8>().unwrap_or(0) );
                        vars.insert(
                            tokens[1].to_owned(),
                            (bin.len() - 1) as u8
                        );
                        println!("\t\t{:?}", vars);
                    },
                    _ => {}
                }
            },

            SEGMENT::Code => {
                match tokens[0].as_str() {
                    "//" => {},
                    "" => {},
                    "seg" => {
                        new_seg(&mut flag, &mut cseg, &tokens[1], bin.len() as i8);
                    },
                    "subleq" => {
                        bin.push(
                            i8_or_var_address(&vars, &tokens[1], "subleq")
                        );
                        bin.push(
                            i8_or_var_address(&vars, &tokens[2], "subleq")
                        );
                        bin.push((bin.len() + 2) as i8);
                        bin.push((bin.len() + 2) as i8);
                        bin.push((bin.len() + 2) as i8);
                        bin.push((bin.len() + 2) as i8);
                    },
                    "goto" => {
                        bin.push(0);
                        bin.push(0);
                        bin.push(
                            i8_or_var_address(&vars, &tokens[1], "goto")
                        );
                    },
                    _ => {}
                }
            },
        }
    }

    // goto Code segment
    bin[2] = cseg as i8 + 1;

    return bin;
}

fn i8_or_var_address(
    vars: &HashMap<String, u8>,
    token: &String,
    ins: &str
) -> i8 {
    token.parse::<i8>().unwrap_or_else(|_| {
        (
            *vars.get(token)
                .expect(&format!("{} as a {} arg is not i8 or var", token, ins))
        ) as i8 + 1
    })
}

fn new_seg(flag: &mut SEGMENT, cseg: &mut i8, token: &String, bin_len: i8) {
    match token.as_str() {
        "data" => {
            *flag = SEGMENT::Data;
        },
        "stack" => {
            *flag = SEGMENT::Stack;
        },
        "code" => {
            *cseg = bin_len;
            *flag = SEGMENT::Code;
        },
        _ => {}
    }
}
