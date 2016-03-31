use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

fn modify_array(wiadomosc: &String, array: &mut [u8]) -> String{
    let (head, tail) = wiadomosc.split_at(8);
    let bytes = head.as_bytes();
    for i in 0..bytes.len() {
        array[i] = (array[i] + bytes[i]) % 128;
    }
    tail.to_string()
}

fn main() {
    println!("Podpis elektroniczny!");

    //create path to directory
    let path = Path::new("wiadomosci.txt");
    let display = path.display();

    // open path in read only mode returns io::Result<File>
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Coudn't open {}: {}", display, Error::description(&err)),
    };

    let file = BufReader::new(&f);

    let mut original_algorytm_array: [u8; 8] = [0; 8];

    let algorytm = "ALGORYTM";
    for (i, c) in algorytm.chars().enumerate() {
        original_algorytm_array[i] = c as u8;
    }

    // println!("{:?}", original_algorytm_array);

    // let mut wiadomosc = String::new();
    for line in file.lines() {
        //pojedyncza wiadomosc
        let mut wiadomosc = line.unwrap();
        let modulo = wiadomosc.len() % 8;
        // println!("{:?}", modulo);
        if modulo != 0 {
            for _ in 0..modulo {
                wiadomosc.push('.');
            }
        }

        let mut algorytm_array = original_algorytm_array;

        let ln = wiadomosc.len();
        for _ in 0..ln/8 {
            // println!("{:?}", wiadomosc);
            wiadomosc = modify_array(&wiadomosc, &mut algorytm_array);
        }

        let mut wynik: [u8; 8] = [0; 8];
        for i in 0..algorytm_array.len() {
            wynik[i] = (65 + (algorytm_array[i] % 26)) as u8;
            print!("{} ", wynik[i]);
        }

        print!("\n");

    }

    let path = Path::new("podpisy.txt");
    let display = path.display();

    // open path in read only mode returns io::Result<File>
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Coudn't open {}: {}", display, Error::description(&err)),
    };

    let file = BufReader::new(&f);

    print!("\n");
    for line in file.lines() {
        let mut podpisy: [u8; 8] = [0; 8];
        let l = line.unwrap();
        let values = l.split(' ');
        for (i, val) in values.enumerate() {
            podpisy[i] = match val.trim().parse() {
                Ok(number) => number,
                Err(_) => panic!("{} is not a number!", val),
            };
            print!("{} ", podpisy[i]);
        }

        print!("\n");


    }
}
