use array2::*;

use std::process;

use crate::args::Args;

// this function calls either rotate_r90 / rotate_c90 / rotate_r180 / rotate_c180 / rotate_r270 / rotate_c270 / rotate_0 function. prints error and exit code if invalid command line arguments
pub fn rotate<T:Clone> (arr2: &mut Array2<T>, args: &mut Args) -> Result<Vec<T>, std::io::Error>{

    if let true = &args.r_fname {

        match &args.rotate{
            Some(90) => {
                arr2.width_height = (arr2.width_height.1, arr2.width_height.0);
                return Ok(rotate_r90(arr2));}
            Some(180) => {return Ok(rotate_r180(arr2));}
            Some(270) => {
                arr2.width_height = (arr2.width_height.1, arr2.width_height.0);
                return Ok(rotate_r270(arr2))}
            Some(0) => {return Ok(rotate_r0(arr2))}
            _ => {
                eprintln!("Invalid rotation degree");
                process::exit(1)},
        }
    } else {
        match &args.c_fname {
            true => {
                match &args.rotate{
                Some(90) => {
                    return Ok(rotate_c90(arr2));}
                Some(180) => {return Ok(rotate_c180(arr2));}
                Some(270) => {return Ok(rotate_c270(arr2))}
                Some(0) => {return Ok(rotate_c0(arr2))}
                _ => {
                    eprintln!("Invalid rotation degree");
                    process::exit(1)},
                }
            }
            _ => {
                eprintln!("No file name entered !");
                process::exit(1)}
        }
    }
}


// These functions takes an array2 struct of type T
// returns a vector of elements of same type in a rotated order, or fliped order or transposed order using either column major or row major iteraton.
pub fn rotate_r90<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_row_major()
        .map(move |x|arr2.get_mut90((x.1, arr2.width_height.0 - x.0 - 1)).unwrap().clone())
        .collect::<Vec<T>>()
}
pub fn rotate_c90<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_col_major()
        .map(move |x|arr2.get_mut((x.0, arr2.width_height.1 - x.1 - 1)).unwrap().clone())
        .collect::<Vec<T>>()
}

pub fn rotate_r180<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_row_major()
        .map(move |x|arr2.get_mut((arr2.width_height.0 - x.0 - 1, arr2.width_height.1 - x.1 - 1)).unwrap().clone())
        .collect::<Vec<T>>()
}
pub fn rotate_c180<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_col_major()
        .map(move |x|arr2.get_mutc((arr2.width_height.0 - x.0 - 1, arr2.width_height.1 - x.1 - 1)).unwrap().clone())
        .collect::<Vec<T>>()
}

pub fn rotate_r270<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{
    arr2.iter_row_major()
    .map(move |x|arr2.get_mut90((arr2.width_height.1 - x.1 - 1, x.0)).unwrap().clone())
    .collect::<Vec<T>>()
}
pub fn rotate_c270<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{
    arr2.iter_col_major()
        .map(move |x|arr2.get_mut((arr2.width_height.0 - x.0 - 1, x.1)).unwrap().clone())
        .collect::<Vec<T>>()
}

pub fn rotate_r0<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{
    arr2.arr.clone()
}
pub fn rotate_c0<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{
    arr2.arr.clone()
}
