/* rustworking_core
 *
 * Rust crate to handle network tasks for
 * system administration.
 *
 * Set of helper functions.
 * Common functions needed by multiple code files.
 *
 * Author: Tim Monfette
 * Version: 0.1.0
 */

use std::sync::mpsc;
use std::process::exit;
use std::io::prelude::*;
use std::io::{BufReader,BufRead};
use std::fs::File;

// Read a file
pub fn read_file(verbose: bool, filepath: String, tx: mpsc::Sender<String>) {
    if verbose {
        println!("Opening file {}", filepath);
    }

    let f = match File::open(&filepath) {
        Ok(file)    => file,
        Err(e)      =>
        {
            let mut stderr = ::std::io::stderr();
            writeln!(&mut stderr, "rustworking: unable to open file {}\nError recieved: {}",
                     filepath, e).expect("Could not write to stderr");
            exit(1);
        },
    };

    if verbose {
        println!("{} successfully opened", filepath);
    }

    let buff = BufReader::new(&f);

    for line in buff.lines() {
        let data = line.unwrap();
        tx.send(data).unwrap();
    }
}

// Process a subnet
pub fn process_subnet(verbose: bool, subnet: String, tx: mpsc::Sender<String>) {
    let vec: Vec<&str> = subnet.split('/').collect();
    let mask = vec[1];
    let address = vec[0];
    let mut octets: Vec<String> = address.split('.').map(String::from).collect();
    let last_octet = octets[3].parse::<i32>().expect("Couldn't get last octet of IP Address");

    let num_host: i32;
    match mask {
        "24" => num_host = 254,
            "25" => num_host = 126,
            "26" => num_host = 62,
            "27" => num_host = 30,
            "28" => num_host = 14,
            "29" => num_host = 6,
            "30" => num_host = 2,
            _    =>
            {
                let mut stderr = ::std::io::stderr();
                writeln!(&mut stderr, "rustworking: invalid Class C mask bits {}",
                         mask).expect("Could not write to stderr");
                exit(1);
            }
    }

    let mut counter = num_host + 1;
    while last_octet > counter {
        counter = counter + num_host + 2;
    }

    // octets now contains the subnet ID
    octets[3] = (counter - num_host - 1).to_string();

    if verbose {
        let mut broadcast = octets.clone();
        broadcast[3] = (broadcast[3].parse::<i32>()
            .expect("Error getting broadcast address") + num_host + 1)
            .to_string();
        println!("Address provided: {}", address);
        println!("Mask bits provided: {}", mask);
        println!("Subnet ID is: {}", octets.join("."));
        println!("Broadcast Address: {}", broadcast.join("."));
        println!("Total number of hosts in subnet: {}", num_host);
        println!("Beginning to ping subnet {}...", subnet);
    }

    // Update last octet to be the first usable address
    octets[3] = (octets[3].parse::<i32>()
        .expect("Couldn't get last octet of IP Address") + 1)
        .to_string();

    for _ in 1..num_host + 1 {
        let addr = octets.join(".");
        tx.send(addr).unwrap();

        octets[3] = (octets[3].parse::<i32>()
            .expect("Couldn't get last octet of IP Address") + 1)
            .to_string();
    }
}
