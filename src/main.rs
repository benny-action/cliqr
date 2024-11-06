use std::io::Write;
/*
* Small tool to build QR codes from the inputted text.
* Little bit of ANSI research to do
* Half block characters, output to terminal.
* Needs function to turn text into QR data, then one to turn it into ANSI
*
* */
#[allow(dead_code)]
struct Input {
    input_string: String,
}

impl Input {
    fn new(input: &str) -> Input {
        Input {
            input_string: input.to_string(),
        }
    }
}

struct QRBody {
    // TODO: add functionality for the control sctructures
    // finder_block:,
    // alignment_block:,
    // timing_pattern:,
    contents: Vec<Input>,
}

impl QRBody {
    fn new() -> QRBody {
        QRBody {
            contents: Vec::new(),
            // finder_block,
            // alignment_block,
            // timing_pattern,
        }
    }
    fn display() {
        // TODO: take contents here,
        // use to_binary on it, store,
        // convert to ansi halfblock chars,
        // wrap in qr block stuff,
        // display depended on sizing.
    }
}

//add capacity for showing on different sized screens - max width, division by

fn main() {
    to_binary();
}

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);

    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    input.trim().to_string()
}

fn to_binary() {
    // TODO: change line below to accept the input
    let string_to_change = "Bilious".to_string();
    let mut string_in_binary = "".to_string();

    for character in string_to_change.clone().into_bytes() {
        string_in_binary += &format!("0{:b} ", character);
    }
    println!("\"{}\" in binary is {}", string_to_change, string_in_binary);
}
