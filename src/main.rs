use std::{cmp::min, time::{SystemTime, UNIX_EPOCH}, vec};



mod seq;
// 4.294.967.295

fn main() {
    println!("Hello, world!");
    let mut i = 10u32;
    while i < 4294967295 {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        for j in 1..10{
            let tot = seq::contar_primos(i.try_into().unwrap());
        }
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        i = min(i*10,4294967295);
    }

}
