#![allow(non_snake_case)]

use std::vec;

fn main() {
    let mut tabInt: [u32;8] = [2,3,4,7,1,5,4,1];
    let tabFloat = [3.14, 2.0, 5.1];
    tabInt.sort();
    print!("4 eme case tabInt : {}\n", tabInt[3]);
    for i in 0..tabFloat.len(){
        print!("case num {} du tabFloat : {}\n", i, tabFloat[i])
    }
    print!("\n");

    let mut vecteur1 = vec![1,2,3,4];
    vecteur1.push(2);
    for i in 0..vecteur1.len(){
        print!("case num {} de vecteur1 : {}\n", i, vecteur1[i])
    }

    print!("\n");
    let tranche  =&tabFloat[0..2];
    for i in 0..tranche.len(){
        print!("case num {} de vecteur1 : {}\n", i, tranche[i])
    }
}
