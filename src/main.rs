use std::io::Write;
use std::vec::Vec;
/*
* Small tool to build QR codes from the inputted text.
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
    fn clear_screen(&mut self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

fn main() {
    let mut qr_translate_app = QRBody::new("");
    qr_translate_app.clear_screen();
    println!("CLIQR: text to QR in command line");
    let new_input = get_string_input("Enter text to become QR: ");
    qr_translate_app.add_text(&new_input);

    let ip_length = new_input.len();

    //debug qr data code printer
    let binary_info = "0100".to_owned() + &to_binary(&new_input); //has byte-mode indicator prepend
    let qr_info = ansi_translate(&binary_info);
    let qr_string = qr_info.to_string();
    println!("binary info output: {}", binary_info);
    println!("string length, module output: {}, {}", ip_length, qr_string);
    println!("{}", qr_info);

    // Create a QR code matrix for version 4 - TODO: pass the string length info to the size
    let qr_matrix = QRCodeMatrix::new(4, &binary_info);

    // Render and print the matrix
    println!("{}", qr_matrix.render());

    // Print total available data modules
    println!("Total Data Modules: {}", qr_matrix.get_total_data_modules());
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
            ' ' => 'x', //this is to stop it crashing when spaces are in the input?
            _ => c,
        })
        .collect()
}
fn smallest_version(string_length: i32) -> (usize, usize) {
    //to determine the smallest possible version of the qr, min level 25x, max level ver40 177x
    //TODO: add match casees for other versions, only have ver 1 here, increment by 4
    let len = string_length;
    match len {
        1..=15 => (24, 24), //this should be 25 but is currently chopping bytes up
        _ => (0, 0),
    }
}
fn output_sizing(dimensions: (usize, usize), content: &str) {
    let (width, height) = dimensions;
    // assert_eq!(content.len(), width * height, "Input must match grid size");

    for row in 0..height {
        for col in 0..width {
            let index = row * width + col;
            let value = content.chars().nth(index).unwrap_or('e');
            print!("{}", value);
        }
        println!();
    }
}

#[derive(Clone, Copy, Default)]
enum ModuleType {
    #[default]
    Empty,
    PositionDetection,
    Timing,
    Alignment,
    DarkModule,
    Reserved,
    Data,
}

pub struct QRCodeMatrix {
    size: usize,
    matrix: Vec<Vec<ModuleType>>,
}

impl QRCodeMatrix {
    /// Create a new QR code matrix for a specific version (size)
    pub fn new(version: usize, data: &str) -> Self {
        // Calculate matrix size based on QR code version
        // QR Code size = (4 * version) + 17
        let size = 4 * version + 17;

        // Initialize matrix with empty modules
        let matrix = vec![vec![ModuleType::Empty; size]; size];

        let mut qr_matrix = QRCodeMatrix { size, matrix };
        let data_modules = data;

        // Add functional patterns
        qr_matrix.add_position_detection_patterns();
        qr_matrix.add_timing_patterns();
        qr_matrix.add_dark_module(version);
        qr_matrix.add_data_modules(data_modules);

        // If version > 1, add alignment patterns
        if version > 1 {
            qr_matrix.add_alignment_patterns(version);
        }

        qr_matrix
    }

    /// main(usr input -> data argument in QRCodeMatrix::new) -> data_modules var -> arg for add_data_mod
    fn add_data_modules(&mut self, data: &str) -> Result<Vec<ModuleType>, &'static str> {
        //Takes a str - converts it to the data module rendered block
        let output = data
            .chars()
            .map(|c| match c {
                '0' => Ok(ModuleType::Empty),
                '1' => Ok(ModuleType::Data),
                _ => Err("Invalid binary character: must be '0' or '1'"),
            })
            .collect();
        //TODO: Make this return correctly. Currently getting tripped up by types, wants to bring
        //the output to the argument within new function, but isn't using render function
        output
    }
    fn add_position_detection_patterns(&mut self) {
        // Top-left
        self.add_position_detection_pattern(0, 0);

        // Top-right
        self.add_position_detection_pattern(0, self.size - 7);

        // Bottom-left
        self.add_position_detection_pattern(self.size - 7, 0);
    }

    // Create a single position detection pattern
    fn add_position_detection_pattern(&mut self, start_row: usize, start_col: usize) {
        for i in 0..7 {
            for j in 0..7 {
                // Create the specific pattern with alternating black and white modules
                let is_border = i == 0 || i == 6 || j == 0 || j == 6;
                let is_inner_border = i == 1 || i == 5 || j == 1 || j == 5;

                if is_border {
                    self.matrix[start_row + i][start_col + j] = ModuleType::PositionDetection;
                } else if is_inner_border {
                    self.matrix[start_row + i][start_col + j] = ModuleType::Empty;
                } else {
                    self.matrix[start_row + i][start_col + j] = ModuleType::PositionDetection;
                }
            }
        }
    }

    // horizontal and vertical timing patterns
    fn add_timing_patterns(&mut self) {
        let pattern_position = 6;

        // Horizontal timing pattern -even numbers filled
        for col in 8..self.size - 8 {
            self.matrix[pattern_position][col] = if col % 2 == 0 {
                ModuleType::Timing
            } else {
                ModuleType::Empty
            };
        }

        // Vertical timing pattern - same again
        for row in 8..self.size - 8 {
            self.matrix[row][pattern_position] = if row % 2 == 0 {
                ModuleType::Timing
            } else {
                ModuleType::Empty
            };
        }
    }

    // Add alignment patterns for versions > 1
    fn add_alignment_patterns(&mut self, version: usize) {
        // Alignment pattern locations based on QR code version
        let locations = self.get_alignment_pattern_locations(version);

        for &(row, col) in &locations {
            self.add_single_alignment_pattern(row, col);
        }
    }

    // Create a single alignment pattern
    fn add_single_alignment_pattern(&mut self, center_row: usize, center_col: usize) {
        let start_row = center_row - 2;
        let start_col = center_col - 2;

        for i in 0..5 {
            for j in 0..5 {
                let is_border = i == 0 || i == 4 || j == 0 || j == 4;
                let is_inner_border = i == 1 || i == 3 || j == 1 || j == 3;

                if is_border {
                    self.matrix[start_row + i][start_col + j] = ModuleType::Alignment;
                } else if is_inner_border {
                    self.matrix[start_row + i][start_col + j] = ModuleType::Empty;
                } else {
                    self.matrix[start_row + i][start_col + j] = ModuleType::Alignment;
                }
            }
        }
    }

    // Determine alignment pattern locations based on QR code version
    fn get_alignment_pattern_locations(&self, version: usize) -> Vec<(usize, usize)> {
        // This is a simplified version. Actual locations depend on the specific QR code version
        match version {
            2..=4 => vec![(self.size / 2, self.size / 2)],
            5..=6 => vec![
                (self.size / 3, self.size / 3),
                (self.size / 3 * 2, self.size / 3 * 2),
            ],
            7..=13 => vec![
                (self.size / 4, self.size / 4),
                (self.size / 4 * 3, self.size / 4),
                (self.size / 4, self.size / 4 * 3),
                (self.size / 4 * 3, self.size / 4 * 3),
            ],
            _ => vec![], // Larger versions would have more complex placement
        }
    }

    // Add the dark module (always at the same location for all versions)
    fn add_dark_module(&mut self, version: usize) {
        let dark_module_row = 4 * version + 9;
        let dark_module_col = 8;
        self.matrix[dark_module_row][dark_module_col] = ModuleType::DarkModule;
    }

    // Render the matrix as a string of block characters
    pub fn render(&self) -> String {
        let mut rendered = String::new();

        for row in &self.matrix {
            for &module in row {
                match module {
                    ModuleType::Empty => rendered.push(' '),
                    ModuleType::PositionDetection => rendered.push('█'),
                    ModuleType::Timing => rendered.push('▓'),
                    ModuleType::Alignment => rendered.push('▒'),
                    ModuleType::DarkModule => rendered.push('▓'),
                    ModuleType::Reserved => rendered.push('░'),
                    ModuleType::Data => rendered.push('·'),
                }
            }
            rendered.push('\n');
        }

        rendered
    }

    /// Get the total number of data modules available - to check against reserved
    pub fn get_total_data_modules(&self) -> usize {
        // This is a simplified estimation
        self.size * self.size - self.count_reserved_modules()
    }

    /// Count the number of reserved/functional modules - to have a total to fit to sizing
    fn count_reserved_modules(&self) -> usize {
        self.matrix
            .iter()
            .flatten()
            .filter(|&&module| {
                matches!(
                    module,
                    ModuleType::PositionDetection
                        | ModuleType::Timing
                        | ModuleType::Alignment
                        | ModuleType::DarkModule
                        | ModuleType::Reserved
                )
            })
            .count()
    }
}
