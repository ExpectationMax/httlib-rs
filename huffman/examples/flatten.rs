extern crate httlib_huffman;

use httlib_huffman::{flatten};
use httlib_huffman::data::encoder::ENCODE_TABLE;

fn main() {
    let speed = 4;
    let matrix = flatten(&ENCODE_TABLE, speed);
    let matrix_len = matrix.len();
    let targets_len = matrix[0].len();

    println!("");
    println!("/// This is a static translation matrix for decoding Huffman sequence by reading");
    println!("/// {}-bit(s) at at time.", speed);
    println!("pub const DECODE_TABLE: [[(Option<usize>, Option<usize>, usize); {}]; {}] = [ // (next_id, ascii, leftover)", targets_len, matrix_len);
    for (i, transitions) in matrix.iter().enumerate() {
        println!("    [ // {}", i);

        for (j, transition) in transitions.iter().enumerate() {
            print!("        {:?}", transition);
            if j != targets_len - 1 {
                print!(",");
            }
            println!("");
        }
        print!("    ]");
        if i != matrix_len - 1 {
            print!(",");
        }
        println!("");
    }
    println!("];");
    println!("");
}
