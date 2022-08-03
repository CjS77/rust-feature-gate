#![feature(proc_macro_hygiene)]

use tari_feature::tari_feature;

#[tari_feature(add_pair)]
mod add_pair;

#[tari_feature(add_pair)]
use add_pair::add_pair;

fn main() {
    println!("Hello world!");
    #[tari_feature(add_pair)]
    println!("40 + 2 = {}", add_pair(40, 2));

    println!("Bye, world!");
}

// generates

/*
#[cfg(tari_add_pair)]
mod add_pair;

#[cfg(tari_add_pair)]
use add_pair::add_pair;

fn main() {
    println!("Hello world!");
    #[cfg(tari_add_pair)]
    println!("40 + 2 = {}", add_pair(40, 2));

    println!("Bye, world!");
}

 */
