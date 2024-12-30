use std::{collections::HashMap, vec};
use lazy_static::lazy_static;

use super::print_utility::print_bitboard;

/* A Chess board has files and ranks */
/* Rank (Row) - horizontal from A to H */
/* Files (Columns) - vertical from 1 to 8*/

lazy_static! {
    static ref RANK_MAP: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('a', 0);
        map.insert('b', 1);
        map.insert('c', 2);
        map.insert('d', 3);
        map.insert('e', 4);
        map.insert('f', 5);
        map.insert('g', 6);
        map.insert('h', 7);
        return map;
    };
    pub static ref SET_MASK: [u64; 64] = {
        let mut arr = [0 as u64; 64];
        for idx in 0..64 {
            arr[idx] |= 1 << idx;
        }
        return arr;
    };
    pub static ref CLEAR_MASK: [u64; 64] = {
        let mut arr = [0 as u64; 64];
        for idx in 0..64 {
            arr[idx] = !(1 << idx);
        }
        return arr;
    };
}

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23, 3, 12, 16, 59, 41, 19, 24, 54, 4, 64, 13, 10, 17, 62,
    60, 28, 42, 30, 20, 51, 25, 44, 55, 47, 5, 32, 64, 38, 14, 22, 11, 58, 18, 53, 63, 9,
    61, 27, 29, 50, 43, 46, 31, 37, 21, 57, 52, 8, 26, 49, 45, 36, 56, 7, 48, 35, 6, 34,
    33,
];

pub fn pop_lsb(mut bitboard: u64) -> usize {
    let idx = bit_scan_lsb(bitboard);
    bitboard ^= (1 << bit_scan_lsb(bitboard));
    return idx;
}

pub fn set_bit(mut bitboard: u64, square: usize) -> u64 {
    bitboard |= SET_MASK[square];
    return bitboard;
}

pub fn clear_bit(mut bitboard: u64, square: usize) -> u64 {
    bitboard &= CLEAR_MASK[square];
    return bitboard;
}

pub fn notation_to_idx(positions: &[&str]) -> Vec<usize> {
    let mut pos_idx_arr = vec![];
    for pos in positions {
        if pos.len() == 2 {
            // println!("{:?}", pos.chars().nth(0).unwrap());
            let file = pos.chars().nth(0).unwrap();
            let rank = pos.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
            let idx = RANK_MAP.get(&file).unwrap() + (rank - 1) * 8;
            pos_idx_arr.push(idx);
        } else {
            panic!("One of the positions is not correct!!!")
        }
    }
    return pos_idx_arr;
}

/**
 TEST: Not sure if the resonse is like that, and not sure if this is for the msb or lsb
 Get least segnificant bit from a bitboard(u64);
 * Ex: get_lsb(bitboard: 0....0111) -> 0
*/
pub fn bit_scan_lsb(bitboard: u64) -> usize {
    // Gets the least significant bit
    let bit: u64 = bitboard ^ (bitboard - 1) ^ (!bitboard & (bitboard - 1));
    return MOD67TABLE[(bit % 67) as usize];
}

/**
 TEST: Not sure if the resonse is like that, and not sure if this is for the msb or lsb
 Get most segnificant bit from a bitboard(u64);
 * Ex: get_msb(bitboard: 0....0111) -> 2
*/
pub fn bit_scan_msb(bitboard: u64) -> usize {
    return (bitboard as f64).log2().floor() as usize;
}

/**
 TEST: Not sure if the resonse is like that
 Extracts all of the bits from a bitboard(u64);
 * Ex: extract_bits(bitboard: 0....0111) -> [0, 1, 2]
*/
pub fn extract_all_bits(mut bitboard: u64) -> Vec<usize> {
    let mut result = vec![];

    while bitboard != 0 {
        let next_bit = bit_scan_lsb(bitboard);
        result.push(next_bit);
        bitboard ^= 1 << next_bit;
    }

    return result;
}

/**
 Sets one bit to the given bitboard(u64) by getting the row and col.
 It also checks if the row and col re in bounds.
 * Ex: set_bit(bitboard: 0, row: 0, col: 1) -> 0...010
*/
// pub fn set_bit(bitboard: u64, row: i8, col: i8) -> u64 {
//     if !is_inside_board_bounds_row_col(row, col) {
//         return bitboard;
//     }
//     return bitboard | (1 << position_to_idx(row, col, None));
// }

/**
 Converts index to row and col tuple.
 If Index is not in bounds it panics if the check_bounds is enabled.
 * Ex: idx_to_position(idx: 50, check_bounds: true) -> (6, 2)
*/
pub fn idx_to_position(index: i8, check_bounds: Option<bool>) -> (i8, i8) {
    let check_bounds = check_bounds.unwrap_or(true);
    if check_bounds && !is_inside_board_bounds_idx(index) {
        panic!("The row and col are not inside bounds");
    }

    return ((index - (index % 8)) / 8, index % 8);
}

/**
 Converts given row and col to position index.
 If row and col are not in bounds it panics if the check_bounds is enabled
 * Ex: position_to_idx(row: 6, col: 2, check_bounds: true) -> 50
*/
pub fn position_to_idx(row: i8, col: i8, check_bounds: Option<bool>) -> i8 {
    let check_bounds = check_bounds.unwrap_or(true);
    if check_bounds && !is_inside_board_bounds_row_col(row, col) {
        panic!("The row and col are not inside bounds");
    }

    return row * 8 + col;
}

/**
Checks if the row and col are inside the board. They should be between 0 and 7 included.
* Ex: is_inside_board_bounds_row_col(row: 8, col: 4) -> false
*/
pub fn is_inside_board_bounds_row_col(row: i8, col: i8) -> bool {
    return 0 <= row && row <= 7 && 0 <= col && col <= 7;
}

/**
Checks if the idx is inside the board. It should be between 0 and 63 included .
* Ex: is_inside_board_bounds_idx(63) -> true
*/
pub fn is_inside_board_bounds_idx(idx: i8) -> bool {
    return 0 <= idx && idx <= 63;
}

// TODO: Needs a rework in the future
// pub fn position_to_bit(position: &str) -> Result<PiecePosition, String> {
//     if position.len() != 2 {
//         return Err(format!(
//             "Invalid length: {}, string: '{}'",
//             position.len(),
//             position
//         ));
//     }

//     let bytes = position.as_bytes();
//     let byte0 = bytes[0];
//     if byte0 < 97 || byte0 >= 97 + 8 {
//         return Err(format!(
//             "Invalid Column character: {}, string: '{}'",
//             byte0 as char, position
//         ));
//     }

//     let column = (byte0 - 97) as u32;

//     let byte1 = bytes[1];
//     let row;
//     match (byte1 as char).to_digit(10) {
//         Some(number) => {
//             if number < 1 || number > 8 {
//                 return Err(format!(
//                     "Invalid Row character: {}, string: '{}'",
//                     byte1 as char, position
//                 ));
//             } else {
//                 row = number - 1;
//             }
//         }
//         None => {
//             return Err(format!(
//                 "Invalid Row character: {}, string: '{}'",
//                 byte1 as char, position
//             ));
//         }
//     }

//     let square_number = row * 8 + column;
//     let bit = (1 as u64) << square_number;

//     Ok(bit)
// }

// DEPRECATE:
// fn bit_scan_simple(mut bit: u64) -> usize {
//     let mut leading_zeros = 0;
//     while (bit & 1) == 0 {
//         bit >>= 1;
//         leading_zeros += 1;
//     }
//     return leading_zeros;
// }

//DEPRECATE:
// pub fn bit_to_position(bit: u64) -> Result<String, String> {
//     if bit == 0 {
//         return Err("No piece present!".to_string());
//     } else {
//         let bit = bit_scan_lsb(bit);
//         return Ok(index_to_position(bit).to_string());
//     }
// }

//DEPRECATE:
// pub fn index_to_position(index: usize) -> String {
//     let file = index / 8 + 1;
//     let rank = index % 8;
//     return format!("{}{}", RANK_MAP[rank], file);
// }

//**** START: TESTS ****
#[cfg(test)]
mod tests {

    use crate::new_engine::print_utility::split_on;

    use super::*;

    #[test]
    fn split_on_space_works() {
        let test_string = "A B C D";
        let (item, rest) = split_on(test_string, ' ');
        assert_eq!(item, "A");
        assert_eq!(rest, "B C D");
    }

    // #[test]
    // fn index_to_position_works() {
    //     let test_arr: [usize; 3] = [1, 10, 62];
    //     assert_eq!(index_to_position(test_arr[0]), "b1");
    //     assert_eq!(index_to_position(test_arr[1]), "c2");
    //     assert_eq!(index_to_position(test_arr[2]), "g8");
    // }

    #[test]
    fn bit_scan_works() {
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan_lsb(bit);
            assert_eq!(i, index);
        }
    }

    #[test]
    fn bit_scan_with_multiple_bits() {
        for lowest_bit in 0..64 {
            let mut bit = 1 << lowest_bit;

            for other_bit in lowest_bit + 1..64 {
                if (other_bit + 37) % 3 != 0 {
                    bit |= 1 << other_bit;
                }
            }
            let bit_scan_result = bit_scan_lsb(bit);
            assert_eq!(lowest_bit, bit_scan_result);
        }
    }

    #[test]
    fn test_get_bits() {
        let bits = (1 << 2) | (1 << 5) | (1 << 55);
        let resp = extract_all_bits(bits);

        assert_eq!(vec![2, 5, 55], resp)
    }
}
//**** END: TESTS ****
