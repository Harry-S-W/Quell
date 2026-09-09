/*
 *  Quell/quellc - An Assembly language program
 *  Copyright (C) 2026  Harry Woodhouse
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://gnu.org>.
 */

 // primitive commands - open, read, write, and close files (like quell files)
 // has basic error handling such as checking for file paths and the existence of files

 // Functions
 .global _open_file
 .global _read_from_file
 .global _write_char_to_file
 .global _close_file
 .global _exit // exit function - just for testing
 .global _error

 .align 4

 _exit:
    mov x0, #0 // status code where 0 is success
    mov x16, #1 // we put the code for sys exit in x16 register 
    svc #0x80 // stop code execution

    ret

_error:
    // mov x0, #-1 // sets x0 to -1 indicating error in program 
    mov x16, #1
    svc #0x80
    ret


 _open_file :
 /* 
    mov x16, #5 // 5 is the ID for opening files, adds 5 to bottom half
    movk x16, #0x200, lsl #16 // movk means to move with keep - adds the values of 2000 to top half

    mov x3, #0x1a4  
    mov x2, x1
    // the path var will be in x0 so we can move it to x1 because the open system call expects it there and if we do not move it then
    // it will be overwritten
    mov x1, x0
    mov x0, #-100 // add valued -100 to register x0 which tells the open system call to search for the path relative to the folder this program is running in
    // mov x2, #0 // stores 0 in register x2 - signifies read only mode

    svc #0x80

    // if x0 is negative the file does not exist. below checks for this condition

    b.cs _error
    ret
*/
    mov x2, #0x1a4
    
    mov x16, #5
    movk x16, #0x200, lsl #16 

    svc #0x80

    // if the carry flag is set, there was an error. x0 will contain the error code.
    b.cs _error
    ret


_read_from_file:

    // register x0 alreadt contains the file ID and x1 already has the buffer address
    mov x2, #1 // We read one byte at a time as the tokeniser only reads one byte at a time
    mov x16, #3
    movk x16, #0x200, lsl #16

    svc #0x80

    b.cs _error

    ret

_write_char_to_file:
    // register x0 alreadt contains the file ID and x1 already has the buffer address

    mov x2, #1 // we can write one byte at a time
    mov x16, #4
    movk x16, #0x200, lsl #16

    svc #0x80

    b.cs _error

    ret

_close_file:
    mov x16, #6
    movk x16, #0x200, lsl #16
    
    svc #0x80

    b.cs _error
    
    ret

