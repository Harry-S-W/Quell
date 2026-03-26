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

// Statements
// Defines the structure of the tokens

pub enum Statements{
    // name: Gate name, target: Qubit(s) as CNOT needs two qubits 
    Gate { 
        kind: GateType, 
        qubits: Vec<String> 
    },

    //
    Measure { 
        qubit: String, 
        target_bit: String 
    },

    //
    Declaration { 
        bit_type: BitType, // "qubit" or "bit"
        name: String 
    },

    // name: Bit type (qubit, bit), target: bit number
    Bits {name: String, target: String},

    // name: punctuation type
    Puncutation {syntax: Punctuation},

    EOF,         
    Illegal(char),
}

pub enum GateType {
    Hadamard,    
    Cnot,       
    Paulix, // - gotta add to lexer
    Phase, // - gotta add to lexer
}

pub enum BitType {
    Qubit(String),   // String because qubits can be name (i.e. left_commissure)
    Bit(String),     // 
}

pub enum Punctuation {
    Semicolon,   // ';'
    Comma,       // ','
}