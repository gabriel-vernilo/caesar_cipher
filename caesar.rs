use std::{usize,isize};

static ALFABETO: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn main(){

println!("type a text to encode with caesar cipher: ");
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let mut text = input;

let mut length = text.chars().count() - 2;

println!("type:\nencode\ndecode\n:  ");
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let mut choice = input;

if  choice.contains("dec") || choice.contains("enc"){
    println!("type the shift: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut shift: isize = input.trim().parse().unwrap();
    if choice.contains("en"){
        encode(length,text,shift);
    }
    else if choice.contains("dec"){
        decode(length,text,shift);
    }
    else{
        println!("invalid choice :/ ");
        main();
    }
}

else{
    println!("invalid choice :/ ");
    main();
}


}

fn encode (length: usize, text:String,shift: isize){

    let mut enc = String::new();

    for i in 0..length{
        if ALFABETO.contains(text.chars().nth(i).unwrap()){
            let mut index:isize =  ALFABETO.chars().position(|r| r == text.chars().nth(i).unwrap()).unwrap() as isize;

            if (index+shift) > 25{
                index = (26 - shift) - index;
                if index <0{
                    index = index*-1;
                }
            }
            else{
                index = index +shift;
            }
            enc.push(ALFABETO.chars().nth((index)as usize).unwrap());
        }
        else{
            enc.push(text.chars().nth(i).unwrap());
        }
    }
    println!("encoded: {}",enc);

}

fn decode(length: usize, text:String,shift: isize){
    let mut dec = String::new();

    for i in 0..length {
        if ALFABETO.contains(text.chars().nth(i).unwrap()){
            let mut index:isize =  ALFABETO.chars().position(|r| r == text.chars().nth(i).unwrap()).unwrap() as isize;
            if index < shift{
                index += 26 - shift;
            }
            else{
                index -= shift;
            }
            dec.push(ALFABETO.chars().nth(index as usize).unwrap());
        }
        else {
            dec.push(text.chars().nth(i).unwrap());
        }
    }
    println!("decoded:\n{} ",dec);

}