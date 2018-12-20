extern crate regex;

mod day_1;
mod day_16;
mod day_19;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod util;

fn main() {
    println!("Merry christmas!");
    day_1::solve();
    day_2::solve();
    day_3::solve();
    day_4::solve();
    day_5::solve();
    day_6::solve();

    day_16::solve();
    
    day_19::solve();
}
