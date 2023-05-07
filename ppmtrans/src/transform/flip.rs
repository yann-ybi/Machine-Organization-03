use array2::*;
use crate::args::Args;
use std::process;

// this function calls either flip_hor / flip_ver / flip_horc / flip_verc function. prints error and exit code if invalid command line arguments
pub fn flip<T:Clone> (arr2: &mut Array2<T>, args: &mut Args) -> Result<Vec<T>, std::io::Error>{

    if let true = &args.r_fname {
        match &args.flip.clone().unwrap() as &str{
            "horizontal" => {return Ok(flip_hor(arr2));}
            "vertical" => {return Ok(flip_ver(arr2));}
            _ => {
                eprintln!("Invalid flip");
                std::process::exit(1)},
        }
    } else {
        if let true = &args.c_fname {
            match &args.flip.clone().unwrap() as &str{
                "horizontal" => {return Ok(flip_horc(arr2));}
                "vertical" => {return Ok(flip_verc(arr2));}
            _ => {
                eprintln!("Invalid flip");
                std::process::exit(1)},
        }
        } else {
            eprintln!("No file name entered !");
            process::exit(1)}
    }
}

// These functions takes an array2 struct of type T
// returns a vector of elements of same type in a flipped order using either column major or row major iteraton.
pub fn flip_ver<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_row_major().map(|x| arr2.get_mut((x.0, arr2.width_height.1 - x.1 - 1)).unwrap().clone()).collect()
}
pub fn flip_hor<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_row_major().map(|x| arr2.get_mut((arr2.width_height.0 - x.0 - 1, x.1)).unwrap().clone()).collect()
}

pub fn flip_verc<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_col_major().map(|x| arr2.get_mut((x.1, arr2.width_height.1 - x.0 - 1)).unwrap().clone()).collect()
}
pub fn flip_horc<T:Clone>(arr2: &mut Array2<T>) -> Vec<T>{

    arr2.iter_col_major().map(|x| arr2.get_mut((arr2.width_height.0 - x.0 - 1, x.1)).unwrap().clone()).collect()
}
