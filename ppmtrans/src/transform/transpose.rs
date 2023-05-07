use array2::*;
use crate::args::Args;
use std::process;

// this function calls either transpose or transposec function. prints an error and exit the program if incorrect command line arguments
pub fn transpose<T:Clone> (arr2: &mut Array2<T>, args: &mut Args) -> Result<Vec<T>, std::io::Error>{
    if let true = &args.r_fname {return Ok(transposer(arr2));} else {
        match &args.c_fname {
                true => {return  Ok(transposec(arr2));}
                _ => {
                    eprintln!("File name missing");
                    std::process::exit(1);
                }
            }
    }
}

// These functions takes an array2 struct of type T
// returns a vector of elements of same type in a transposed order using either column major or row major iteraton.

pub fn transposer<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{
    arr2.iter_col_major().map(|x| arr2.get_mut(x).unwrap().clone()).collect()
}
pub fn transposec<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{
    arr2.iter_col_major().map(|x| arr2.get_mut(x).unwrap().clone()).collect()
}