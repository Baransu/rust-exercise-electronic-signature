use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::str;

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

    let path = Path::new("wiadomosci.txt");
    let display = path.display();
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

    let mut output_string = String::from("78.1 \n");

    let mut lines = String::new();

    for (i, line) in file.lines().enumerate() {
        //pojedyncza wiadomosc
        let mut wiadomosc = line.unwrap();
        let modulo = wiadomosc.len() % 8;

        for _ in 0..8 - modulo {
            wiadomosc.push('.');
        }

        if i == 0 {
            output_string.push_str("a) ");
            let dlugosc = wiadomosc.len();
            // println!("{:?}", dlugosc);
            output_string.push_str(&dlugosc.to_string()[..]);
            output_string.push_str("\nb) ");
        }

        let mut algorytm_array = original_algorytm_array;

        let ln = wiadomosc.len();
        for _ in 0..ln/8 {
            wiadomosc = modify_array(&wiadomosc, &mut algorytm_array);
        }

        let mut wynik: [u8; 8] = [0; 8];
        for x in 0..algorytm_array.len() {
            wynik[x] = (65 + (algorytm_array[x] % 26)) as u8;
            if i == 0 {
                output_string.push_str(&wynik[x].to_string()[..]);
                output_string.push(' ');
            }
            // print!("{} ", wynik[x]);
        }

        if i == 0 {
            output_string.push_str("\nc) ");
            output_string.push_str(str::from_utf8(&wynik).unwrap());
            // for x in 0..wynik.len() {
            // }
        }

        lines.push_str(str::from_utf8(&wynik).unwrap());
        lines.push('\n');
        // print!("\n");
    }

    let path = Path::new("podpisy.txt");
    let display = path.display();

    // open path in read only mode returns io::Result<File>
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Coudn't open {}: {}", display, Error::description(&err)),
    };

    let file = BufReader::new(&f);
    // print!("\n");

    let mut to_check_string = String::new();

    output_string.push_str("\n\n78.2 \n");
    for line in file.lines() {
        let mut podpisy: [u8; 8] = [0; 8];
        let l = line.unwrap();
        let values = l.split(' ');
        for (i, val) in values.enumerate() {
            podpisy[i] = match val.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("{} is not a number!", val),
            };
            let a = podpisy[i] as u64 * 3 % 200;
            podpisy[i] = a as u8;
            // print!("{} ", podpisy[i]);
        }

        // print!("\n");
        to_check_string.push_str(str::from_utf8(&podpisy).unwrap());
        to_check_string.push_str("\n");
        output_string.push_str(str::from_utf8(&podpisy).unwrap());
        output_string.push_str("\n");

    }

    output_string.push_str("\n78.3 \n");
    for (i, l1) in to_check_string.lines().enumerate() {
        for (j, l2) in lines.lines().enumerate() {
            if i != j { continue; }
            else if l1 == l2 {
                let index = i + 1;
                output_string.push_str(&index.to_string()[..]);
                output_string.push(' ');
            }
        }
    }

    // println!("{:?}", output_string);

    let path = Path::new("epodpis_wynik.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
    };
    match file.write_all(output_string.as_bytes()) {
        Ok(_) => println!("Zapisano wynik do {}", display),
        Err(why) => panic!("Couldn't write to {}: {}", display, Error::description(&why)),
    }
}
