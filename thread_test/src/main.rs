use rayon::prelude::*;
use std::{thread,time};
use std::thread::sleep;

fn main(){

    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("hdd spawn1 i = {}", i);
            sleep(time::Duration::new(1,0));
        }
    });

    rayon::scope(|s| {
        s.spawn(|_| {
            println!("spawn2 begin");
            for i in 1..10{
                println!("hdd spawn2 i = {}", i);
            }
        });
    });

    println!("spawn1 over");
    handle.join();
    //sleep(time::Duration::new(1,0));
    //branch 2 
    println!("hdd main over");
}
