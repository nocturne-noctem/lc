///
/// lc - line counter
///
/// lc counts the lines of the specified files.
/// lc is in many ways like wc, but in rust. Yay.
/// 
/// Author: Kristian Cordes-Andersen
///


use std::env;

extern crate lc;

fn main() {
   let mut args: Vec<String> = env::args().collect();

   args.remove(0);

   if args.len() > 0 {
       for arg in args {
           lc::run(&arg);
       }
   } else {
       println!("No input supplied.");
   }
}
