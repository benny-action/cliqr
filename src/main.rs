use std::io::Write;
/*
* Small tool to build QR codes from the inputted text.
* Little bit of ANSI research to do
* Half block characters, output to terminal.
* Needs function to turn text into QR data, then one to turn it into ANSI
*
* */
struct QRBody {
    contents: String,
}

impl QRBody {
    fn new(initial_text: &str) -> QRBody {
        QRBody {
            contents: initial_text.to_string(),
            // finder_block,
            // alignment_block,
            // timing_pattern,
        }
    }
    fn add_text(&mut self, input_text: &str) {
        self.contents.push_str(input_text);
    }
    //fn show_qr(&mut self) {
    //    let blocks = ansi_translate(to_binary(&self.contents));
    //    return blocks;
    //}
    fn clear_screen(&mut self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
//TODO: add: error correction level,
//qr code version used, character count indicator,
//padding

fn main() {
    let mut qr_translate_app = QRBody::new("");
    qr_translate_app.clear_screen();
    println!("CLIQR: text to QR in command line");
    let new_input = get_string_input("Enter text to become QR: ");
    qr_translate_app.add_text(&new_input);

    let ip_length = new_input.len();
    println!("{}", ip_length); //output to version function to get info for sizing

    //QR code printer
    let binary_info = "0100".to_owned() + &to_binary(&new_input); //has byte-mode indicator prepend
    let qr_info = ansi_translate(&binary_info);
    //TODO: Refactor to reduce mem footprint
    println!("{}", binary_info);
    println!("{}", qr_info);
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

fn to_binary(string_to_change: &str) -> String {
    let string_to_change = string_to_change;
    let mut string_in_binary = "".to_string();

    for character in string_to_change.bytes() {
        string_in_binary += &format!("0{:b}", character);
    }
    return string_in_binary;
}
fn ansi_translate(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '1' => '█',
            '0' => ' ',
            _ => c,
        })
        .collect()
}
fn error_correction() {
    //function to inplement M level error correction - to recover up to 15% of the data, just use a
    //reed solomon crate
}
fn smallest_version(string_length: i32) -> (i32, i32) {
    //to determine the smallest possible version of the qr, min level 25x, max level ver40 177x
    //TODO: add match casees for other versions, only have ver 1 here, increment by 4
    let len = string_length;
    match len {
        1..=15 => (25, 25),
        _ => (0, 0),
    }
}
