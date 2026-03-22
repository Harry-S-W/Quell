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

use quell::{Lexer, Token};

// this is a small test :)
// passing a string of Quell code to the tokenization system
fn main() {
    let input = "h q0; cx q0, q1; m q0;".to_string();
    let mut lexer = Lexer::new(input);

    loop {
        let tok = lexer.next_token();
        println!("Token: {:?}", tok);
        if tok == Token::EOF { break; }
    }
}