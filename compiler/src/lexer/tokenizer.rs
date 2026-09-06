/*
This file is part of Quell.

Quell is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

Quell is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
Quell. If not, see <https://www.gnu.org/licenses/>.
*/

/// Basic enum for tokenization of .qll files

// this is absolutelty not exhaustive 
// it also does not allow anything besides the most basic functions (qubits, bits, gates, and simple syntax)
// which will change when i get a basic parser working

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Hadamard,    // 'h'
    Cnot,        // 'cx'
    Measure,     // 'm'
    Qubit(u8),   // 'q0'
    Bit(u8),     // 'b0'
    Semicolon,   // ';'
    Comma,       // ','
    EOF,         // end of file
    Illegal(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];
        self.position += 1;

        match ch {
            'h' => Token::Hadamard,
            'm' => Token::Measure,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            'c' => {
                if self.peek_char() == 'x' {
                    self.position += 1;
                    Token::Cnot
                } else {
                    Token::Illegal('c')
                }
            }
            'q' => self.read_number_token(true),
            'b' => self.read_number_token(false),
            _ => Token::Illegal(ch),
        } 
    }

    // helper to look at the next char without moving the position
    fn peek_char(&self) -> char {
        if self.position >= self.input.len() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    // helper to read digits after 'q' or 'b' for qubit and bit
    fn read_number_token(&mut self, is_qubit: bool) -> Token {
        let mut res = String::new();
        while self.peek_char().is_ascii_digit() {
            res.push(self.input[self.position]);
            self.position += 1;
        }
        let num = res.parse::<u8>().unwrap_or(0);
        if is_qubit { Token::Qubit(num) } else { Token::Bit(num) }
    }

    // skipping white space (as the name suggests lol)
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }
}