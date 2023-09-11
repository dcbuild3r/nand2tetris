use regex::Regex;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug)]
pub struct CInstruction {
    pub dest: String,
    pub comp: String,
    pub jmp: String,
}

#[derive(Debug)]
pub enum Instruction {
    A { address: usize },
    C { instruction: CInstruction },
}

/// # Assembler
/// Assembler struct that holds all the state needed to assemble a .asm file
pub struct Assembler<'a> {
    /// input string
    input: &'a str,
    /// iterator over lines of input
    iterator: Lines<'a>,
    /// current instruction
    current_instruction: Option<String>,
    /// current line number
    current_line: usize,
    /// next address to assign to a variable
    next_symbol_address: usize,
    /// symbol table mapping symbols to addresses
    symbol_table: HashMap<String, usize>,
    /// compute instruction mapping to binary representation
    compute_hash_map: HashMap<String, String>,
    /// destination instruction mapping to binary representation
    dest_hash_map: HashMap<String, String>,
    /// jump instruction mapping to binary representation
    jump_hash_map: HashMap<String, String>,
    /// l_instruction matches (LABEL)
    l_instruction: Regex,
    /// a_instruction matches @ADDRESS
    a_instruction: Regex,
    /// c_instruction matches DEST=COMP;JMP
    c_instruction: Regex,
}

impl<'a> Assembler<'a> {
    /// `Assembler.new()`: constructor
    pub fn new(input: &'a str) -> Self {
        // maps all the built-in symbols
        let mut symbol_table = HashMap::new();
        symbol_table.insert("SP".to_string(), 0);
        symbol_table.insert("LCL".to_string(), 1);
        symbol_table.insert("ARG".to_string(), 2);
        symbol_table.insert("THIS".to_string(), 3);
        symbol_table.insert("THAT".to_string(), 4);
        symbol_table.insert("R0".to_string(), 0);
        symbol_table.insert("R1".to_string(), 1);
        symbol_table.insert("R2".to_string(), 2);
        symbol_table.insert("R3".to_string(), 3);
        symbol_table.insert("R4".to_string(), 4);
        symbol_table.insert("R5".to_string(), 5);
        symbol_table.insert("R6".to_string(), 6);
        symbol_table.insert("R7".to_string(), 7);
        symbol_table.insert("R8".to_string(), 8);
        symbol_table.insert("R9".to_string(), 9);
        symbol_table.insert("R10".to_string(), 10);
        symbol_table.insert("R11".to_string(), 11);
        symbol_table.insert("R12".to_string(), 12);
        symbol_table.insert("R13".to_string(), 13);
        symbol_table.insert("R14".to_string(), 14);
        symbol_table.insert("R15".to_string(), 15);
        symbol_table.insert("SCREEN".to_string(), 16384);
        symbol_table.insert("KBD".to_string(), 24576);

        // maps all the built-in compute instructions that the ALU supports
        let mut compute_hash_map = HashMap::new();
        compute_hash_map.insert("0".to_string(), "0101010".to_string());
        compute_hash_map.insert("1".to_string(), "0111111".to_string());
        compute_hash_map.insert("-1".to_string(), "0111010".to_string());
        compute_hash_map.insert("D".to_string(), "0001100".to_string());
        compute_hash_map.insert("A".to_string(), "0110000".to_string());
        compute_hash_map.insert("M".to_string(), "1110000".to_string());
        compute_hash_map.insert("!D".to_string(), "0001101".to_string());
        compute_hash_map.insert("!A".to_string(), "0110001".to_string());
        compute_hash_map.insert("!M".to_string(), "1110001".to_string());
        compute_hash_map.insert("-D".to_string(), "0001111".to_string());
        compute_hash_map.insert("-A".to_string(), "0110011".to_string());
        compute_hash_map.insert("-M".to_string(), "1110011".to_string());
        compute_hash_map.insert("D+1".to_string(), "0011111".to_string());
        compute_hash_map.insert("A+1".to_string(), "0110111".to_string());
        compute_hash_map.insert("M+1".to_string(), "1110111".to_string());
        compute_hash_map.insert("D-1".to_string(), "0001110".to_string());
        compute_hash_map.insert("A-1".to_string(), "0110010".to_string());
        compute_hash_map.insert("M-1".to_string(), "1110010".to_string());
        compute_hash_map.insert("D+A".to_string(), "0000010".to_string());
        compute_hash_map.insert("D+M".to_string(), "1000010".to_string());
        compute_hash_map.insert("D-A".to_string(), "0010011".to_string());
        compute_hash_map.insert("D-M".to_string(), "1010011".to_string());
        compute_hash_map.insert("A-D".to_string(), "0000111".to_string());
        compute_hash_map.insert("M-D".to_string(), "1000111".to_string());
        compute_hash_map.insert("D&A".to_string(), "0000000".to_string());
        compute_hash_map.insert("D&M".to_string(), "1000000".to_string());
        compute_hash_map.insert("D|A".to_string(), "0010101".to_string());
        compute_hash_map.insert("D|M".to_string(), "1010101".to_string());

        // maps all the built-in dest instructions that the ALU supports
        let mut dest_hash_map = HashMap::new();
        dest_hash_map.insert("null".to_string(), "000".to_string());
        dest_hash_map.insert("".to_string(), "000".to_string());
        dest_hash_map.insert("M".to_string(), "001".to_string());
        dest_hash_map.insert("D".to_string(), "010".to_string());
        dest_hash_map.insert("MD".to_string(), "011".to_string());
        dest_hash_map.insert("A".to_string(), "100".to_string());
        dest_hash_map.insert("AM".to_string(), "101".to_string());
        dest_hash_map.insert("AD".to_string(), "110".to_string());
        dest_hash_map.insert("AMD".to_string(), "111".to_string());

        // maps all the built-in jump instructions that the ALU supports
        let mut jump_hash_map = HashMap::new();
        jump_hash_map.insert("null".to_string(), "000".to_string());
        jump_hash_map.insert("".to_string(), "000".to_string());
        jump_hash_map.insert("JGT".to_string(), "001".to_string());
        jump_hash_map.insert("JEQ".to_string(), "010".to_string());
        jump_hash_map.insert("JGE".to_string(), "011".to_string());
        jump_hash_map.insert("JLT".to_string(), "100".to_string());
        jump_hash_map.insert("JNE".to_string(), "101".to_string());
        jump_hash_map.insert("JLE".to_string(), "110".to_string());
        jump_hash_map.insert("JMP".to_string(), "111".to_string());

        // regexes for the three types of instructions

        // l_instruction matches (LABEL)
        let l_instruction: Regex = Regex::new(r"^\(([_0-9a-zA-Z\.\$:]+)\)").unwrap();
        // a_instruction matches @ADDRESS
        let a_instruction: Regex = Regex::new(r"^@([_0-9a-zA-Z\.\$:]+)").unwrap();
        // c_instruction matches DEST=COMP;JMP
        let c_instruction: Regex =
            Regex::new(r"^([ADM]*)(=?)([-\+01DAM!&\|]+)(;?)([JGTEQNLMP]*)").unwrap();

        Assembler {
            input: input,
            iterator: input.lines(),
            current_instruction: None,
            current_line: 0,
            next_symbol_address: 16,
            symbol_table: symbol_table,
            compute_hash_map: compute_hash_map,
            dest_hash_map: dest_hash_map,
            jump_hash_map: jump_hash_map,
            l_instruction: l_instruction,
            a_instruction: a_instruction,
            c_instruction: c_instruction,
        }
    }

    /// `Assembler.reset_input_iterator()`: resets the input iterator when the reset button is pressed
    /// on the Hack computer
    fn reset_input_iterator(&mut self) {
        self.current_line = 0;
        self.iterator = self.input.lines();
    }

    /// public `Assembler.assemble()`: assembles the input asm instruction and returns the binary representation
    pub fn assemble(&mut self) -> String {
        self.build_symbol_table();
        self.reset_input_iterator();

        let mut output = String::new();
        self.advance();
        while self.current_instruction != None {
            if let Some(instruction) = self.get_instruction() {
                output += &format!("{}\n", self.get_machine_language_instruction(instruction));
            }
            self.advance();
        }
        output
    }

    /// `Assembler.build_symbol_table()`: builds the symbol table from @variable declarations
    fn build_symbol_table(&mut self) {
        let mut line_counter = 0;

        self.advance();
        while self.current_instruction != None {
            if let Some(symbol) = self.get_l_symbol() {
                self.symbol_table.insert(symbol, line_counter);
            } else {
                line_counter += 1;
            }
            self.advance();
        }
    }

    /// `Assembler.get_l_symbol()`: returns the symbol name from a l_instruction e.g. (LABEL)
    fn get_l_symbol(&mut self) -> Option<String> {
        let c = self.current_instruction.as_ref().unwrap();
        if self.l_instruction.is_match(c) {
            let caps = self.l_instruction.captures(c).unwrap();
            let symbol_name: String = caps.get(1).map_or("", |m| m.as_str()).to_string();
            Some(symbol_name)
        } else {
            None
        }
    }

    /// `Assembler.get_machine_language_instruction`: Converts DEST=COMP;JMP to a 16 bit binary representation of CInstruction the ALU understands
    fn get_machine_language_instruction(&self, instruction: Instruction) -> String {
        match instruction {
            Instruction::A { address } => {
                let s = format!("{:b}", address);
                format!("{:0>16}", s)
            }
            Instruction::C { instruction } => {
                format!(
                    "111{comp}{dest}{jump}",
                    comp = self.compute_hash_map[&instruction.comp],
                    dest = self.dest_hash_map[&instruction.dest],
                    jump = self.jump_hash_map[&instruction.jmp]
                )
            }
        }
    }

    fn advance(&mut self) {
        if let Some(line) = self.iterator.next() {
            // increase the current line number
            self.current_line += 1;

            // create substring without comments
            let mut s: String = line.to_string();
            s = str::replace(&s, " ", "");
            let comment_offset = s.find("//").unwrap_or(s.len());
            let instruction: String = s.drain(..comment_offset).collect();

            if instruction.is_empty() {
                self.advance();
            } else {
                self.current_instruction = Some(instruction);
            }
        } else {
            self.current_instruction = None;
        }
    }

    /// converts the current instruction into an Instruction enum.
    fn get_instruction(&mut self) -> Option<Instruction> {
        // take the current instruction
        let c = self.current_instruction.as_ref().unwrap();

        // if the current instruction is a c_instruction, parse it and return a CInstruction
        if self.c_instruction.is_match(c) {
            let caps = self.c_instruction.captures(c).unwrap();
            let dest = caps.get(1).map_or("", |m| m.as_str());
            let comp = caps.get(3).map_or("", |m| m.as_str());
            let jmp = caps.get(5).map_or("", |m| m.as_str());
            Some(Instruction::C {
                instruction: CInstruction {
                    dest: dest.to_string(),
                    comp: comp.to_string(),
                    jmp: jmp.to_string(),
                },
            })
        // if the current instruction is an a_instruction, parse it and return an AInstruction
        } else if self.a_instruction.is_match(c) {
            let caps = self.a_instruction.captures(c).unwrap();
            let address_or_symbol: String = caps.get(1).map_or("", |m| m.as_str()).to_string();

            let address_number = match address_or_symbol.parse::<usize>() {
                Ok(number) => number,
                _ => {
                    // if the symbol is not in the symbol table, add it
                    if !self.symbol_table.contains_key(&address_or_symbol) {
                        self.symbol_table
                            .insert(address_or_symbol.clone(), self.next_symbol_address);
                        self.next_symbol_address += 1;
                    }

                    self.symbol_table[&address_or_symbol]
                }
            };
            // return the AInstruction
            Some(Instruction::A {
                address: address_number,
            })
        // if the current instruction is a l_instruction, return None
        } else if self.l_instruction.is_match(c) {
            None
        } else {
            println!(
                "Assembler failed. Syntax error at line {} of the input file.",
                self.current_line
            );
            ::std::process::exit(1);
        }
    }
}
