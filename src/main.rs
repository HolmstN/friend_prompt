use std::io;
use std::{thread, time};

fn wait_with_dots(time: u64) {
    let wait_time = time::Duration::from_millis(time);

    println!{"."};
    thread::sleep(wait_time);
    println!{"."};
    thread::sleep(wait_time);
    println!{"."};
    thread::sleep(wait_time);
} 

fn main() {
    println!("Enter your name: ");
    let mut name = String::new();
    
    io::stdin().read_line(&mut name)
        .expect("Could not read name");
    
    wait_with_dots(500);
    println!{"Welcome, {}", name};
    
    println!{"You have been invited to partake in a new mission.  Do you accept?"};
    println!{"type 'y'"};
    
    loop {
        let mut response = String::new();
        
        io::stdin().read_line(&mut response)
            .expect("Could not read response");
        
        match response.trim() {
            "Y" | "y" => break,
            _ => println!{"You must accept the mission.  Please type 'y'?"},
        }
    }
    
    println!("{}, would you join me for my wedding as a groomsman?", name.trim());
    println!("Please email with your response:  nrholmstedt@gmail.com");
    println!("Done?  Enter anything.");
    loop {
        let mut response = String::new();
        
        io::stdin().read_line(&mut response)
            .expect("Could not read response");
        
        break;
    }
}
