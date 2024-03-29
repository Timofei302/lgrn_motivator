use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut stdout: std::io::Stdout = stdout();

    let line:String = String::from("legro, where are the arts");
    let symbols:String = String::from(" ,.qwertyuiopasdfghjklzxcvbnm");

    loading("Program init");
    loading("Checking the network");
    loading("Connecting to the server");
    loading("Preparing for the transfer");

    println!();
    for i in 0..line.chars().count() {
        for j in 0..symbols.chars().count() {
            print!("\r{}{}", &line[0..i], &symbols.chars().nth(j).unwrap());

            stdout.flush().unwrap();
            sleep(Duration::from_millis(100));
            if line.chars().nth(i).unwrap() == symbols.chars().nth(j).unwrap() {
                break;
            }
        }
    }
    println!("\n");
    sleep(Duration::from_secs(1));

    println!("The program was developed by Timofei302 on Rust with love");
    sleep(Duration::from_secs(1));
    println!("#TogetherWithLGRNArts");
    sleep(Duration::from_secs(1));
}

fn loading(input:&str) {
    let mut stdout: std::io::Stdout = stdout();
    let anim:[char; 5] = ['-', '\\', '|', '/', '√'];
    let mut n:usize = 0;

    for i in 0..=20 {
        if i == 20 {
            n = 4;
        } else if n == 4{
            n = 0;
        }

        print!("\r{}... {}", input, anim[n]);

        stdout.flush().unwrap();
        sleep(Duration::from_secs_f32(0.2));

        n += 1;
    }
    println!();

    sleep(Duration::from_secs(2));
}