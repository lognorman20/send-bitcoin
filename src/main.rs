use std::io;
use rand::Rng;

fn receive_bitcoin() {
    println!("Receiving some Bitcoin!");
    let amount = rand::thread_rng().gen_range(1, 10);
    println!("You just received {} Bitcoin!", amount);
}

fn send_bitcoin() {
    println!("Sending some Bitcoin!");
    let clients = vec!["Obama", "Trump", "Biden", "Clinton"];
    println!("Who do you want to send some Bitcoin to?");
    // the &clients only references the values of the vector. this
    // is called borrowing in rust
    for client in &clients {
        print!("{} ", client);
    }

    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient);

    if clients.contains(&recipient.trim()) {
        println!("How many Bitcoin do you want to send?");

        let mut amount = String::new();
        io::stdin().read_line(&mut amount);

        println!("You sent {} Bitcoin to {}!", amount, recipient);
    }
    println!("");
}

fn exit_console() {
    println!("Invalid option, must be (s) or (r).");
}

fn console() {
    println!("Do you want to send (s) or receive (r)?");

    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("s") {
        send_bitcoin()
    } else if command.trim().eq("r") {
        receive_bitcoin()
    } else {
        exit_console()
    }
}

fn main() {
    console();
}