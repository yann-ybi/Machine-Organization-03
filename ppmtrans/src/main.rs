#![allow(unused)]

pub mod args;
mod transform; // contains rotate / flip and transpose funcitons


use crate::transform::rotate::*;
use crate::transform::flip::*;
use crate::transform::transpose::*;

use csc411_image::{RgbImage, Rgb, Read, Write};
use std::process;
use array2::*;
use std::env;
use args::Args;
use clap::{Parser, builder::Str};


fn main() {

    let mut args = Args::parse();

    construct_rotated_rgbimage(&mut args).write(None);

}
// reads the file name, makes an array2 struct out of it, then calls flip_rotate_transpose
// returns a rotated RgbImage structure
fn construct_rotated_rgbimage(args: &mut Args) -> RgbImage{  

    let mut img = RgbImage::read(args.fname.as_deref()).unwrap();  
    let mut arr2 = Array2::new(img.pixels,(img.width as usize, img.height as usize));

    let new_gi = RgbImage{ 
    pixels:flip_rotate_transpose(&mut arr2, args).unwrap(), 
    width: match &args.rotate{
        Some(90) | Some(270) => img.height,
        _ => match &args.transpose {
            true => img.height,
            false =>img.width,
        }
    },
    height: match &args.rotate{
        Some(90) | Some(270) => img.width,
        _ => match &args.transpose {
            true => img.width,
            false =>img.height,
        }
    },
    denominator: img.denominator};
    new_gi
}

// this function calls either flip, rotate or transpose function. prints an error and exit the program if incorrect command line arguments
pub fn flip_rotate_transpose<T:Clone>(arr2: &mut Array2<T>, args: &mut Args) -> Result<Vec<T>, std::io::Error>{ 

    if let Some(_f) = &args.flip {flip(arr2, args)} else {
        match &args.rotate {
            Some(_r) => rotate(arr2, args),
            None => match &args.transpose{
                true => {transpose(arr2, args)}
                false => {
                    eprintln!("Invalid fags: flip/rotate/transpose entered");
                    std::process::exit(1);
                }
            }
        }
    }
}

// rotate, flip and transpose are implemented inside other crates inside of my transform folder