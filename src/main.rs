use std::io::Write;
use std::vec::Vec;
/*
* Build QR codes from text.
*
* */
fn main() {
    clear_screen();
    println!("CLIQR: text to QR in command line");
    let new_input = get_string_input("Enter text to become QR: ");

    let ip_length = new_input.len();
    println!("debug input length: {}", ip_length);

    // Debug qr data code printer may need byte-mode indicator prepend
    let binary_info = to_binary(&new_input);

    println!("______________________________________________________________________________");
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

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[derive(Clone, Copy, Default, Debug)]
enum ModuleType {
    #[default]
    Empty,
    PositionDetection,
    Timing,
    Alignment,
    DarkModule,
    Data,
}

pub struct QRCodeMatrix {
    size: usize,
    matrix: Vec<Vec<ModuleType>>,
}

impl QRCodeMatrix {
    // Create a new QR code matrix for a specific version (size)
    pub fn new(version: usize, info: &str) -> Self {
        // Calculate matrix size based on QR code version
        // QR Code size = (4 * version) + 17
        let size = 4 * version + 17;

        // Init matrix with empty modules
        let matrix = vec![vec![ModuleType::Empty; size]; size];

        let mut qr_matrix = QRCodeMatrix { size, matrix };

        // Add functional patterns
        let data_vec = qr_matrix.add_data_modules(info);
        qr_matrix.data_module_positioning(data_vec);
        qr_matrix.add_position_detection_patterns();
        qr_matrix.add_timing_patterns();
        qr_matrix.add_dark_module(version);

        // If version > 1, add alignment patterns
        if version > 1 {
            qr_matrix.add_alignment_patterns(version);
        }
        //data debug printing
        //println!("{:?}", &data_vec);
        qr_matrix
    }

    fn add_data_modules(&mut self, binary: &str) -> Vec<ModuleType> {
        binary
            .chars()
            .filter(|&c| c == '0' || c == '1')
            .map(|c| match c {
                '0' => ModuleType::Empty,
                '1' => ModuleType::Data,
                _ => unreachable!("Filtered out non-binary characters"),
            })
            .collect()
    }

    fn data_module_positioning(&mut self, vectorised_data: Vec<ModuleType>) {
        //TODO: add masking in order to not place under the functional parts.
        //TODO: fix out of bounds error - only printing to inner vec, 32 bits and under type thing
        let mut row = 0;
        let mut col = 0;
        for (index, state) in vectorised_data.iter().enumerate() {
            if let ModuleType::Data = state {
                self.matrix[row][col] = ModuleType::Data;
            } else {
                self.matrix[row][col] = ModuleType::Empty;
            }
            col += 1;

            if index % (8 * 4) == 0 {
                // TODO: just hardcoded 4 for current size, make flexible
                row += 1
            }

            if index % (8 * 4) == 0 {
                // TODO: just hardcoded 4 for current size, make flexible
                col -= col
            }
        }
    }
    fn add_position_detection_patterns(&mut self) {
        // Top-left
        self.add_position_detection_pattern(0, 0);

        // Top-right
        self.add_position_detection_pattern(0, self.size - 7);

        // Bottom-left
        self.add_position_detection_pattern(self.size - 7, 0);
    }

    // single position detection pattern
    fn add_position_detection_pattern(&mut self, start_row: usize, start_col: usize) {
        for i in 0..7 {
            for j in 0..7 {
                // pattern with alternating black and white modules
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

    // single alignment pattern
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
            _ => vec![], // Larger versions more complex placement
        }
    }

    // Add the dark module
    fn add_dark_module(&mut self, version: usize) {
        let dark_module_row = 4 * version + 9;
        let dark_module_col = 8;
        self.matrix[dark_module_row][dark_module_col] = ModuleType::DarkModule;
    }

    // Render the matrix as a string of block characters
    pub fn render(&self) -> String {
        let mut rendered = String::new(); //here becomes string

        for row in &self.matrix {
            for &module in row {
                match module {
                    ModuleType::Empty => rendered.push(' '),
                    ModuleType::PositionDetection => rendered.push('█'),
                    ModuleType::Timing => rendered.push('▓'),
                    ModuleType::Alignment => rendered.push('▒'),
                    ModuleType::DarkModule => rendered.push('▓'),
                    ModuleType::Data => rendered.push('█'),
                }
            }
            rendered.push('\n');
        }

        rendered
    }

    // Get the total number of data modules available - to check against reserved
    pub fn get_total_data_modules(&self) -> usize {
        // This is a simplified estimation
        self.size * self.size - self.count_reserved_modules()
    }

    // Count the number of reserved/functional modules - to have a total to fit to sizing
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
                )
            })
            .count()
    }
}
//TESTS
