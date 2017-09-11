/* rustworking-core
 *
 * Rust crate to handle network tasks for
 * system administration.
 *
 * Set of functions to hanlde testing IP Addresses
 * using PING.
 *
 * Author: Tim Monfette
 * Version: 0.1.0
 */

use oping::{Ping, PingResult, PingError};

use std::process::exit;
use std::io::prelude::*;
use std::io::{BufReader,BufRead};
use std::fs::File;

// PING a single IP Address
pub fn ping_ip(verbose: bool, ip: &str) -> PingResult<(String)> {
    send_ping(verbose, ip)
}

// PING an entire subnet of IP Addresses
// Currently limited to Class C subnets ONLY
//      This means your mask bits can be 24 - 30
pub fn ping_subnet(verbose: bool, subnet: &str) -> Vec<PingResult<(String)>> {
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
        _  =>
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

    let mut results = Vec::new();
    for _ in 1..num_host + 1 {
        let addr = &octets.join(".");
        if verbose {
            println!("Sending PING for address {}", addr);
        }

        results.push(send_ping(verbose, addr));

        if verbose {
            println!("PING for address {} completed", addr);
        }

        octets[3] = (octets[3].parse::<i32>()
                     .expect("Couldn't get last octet of IP Address") + 1)
                     .to_string();
    }

    if verbose {
        println!("Returning all PING responses to caller...");
    }

    results
}

// PING every IP Address in a file
pub fn ping_file(verbose: bool, filepath: &str) -> Vec<PingResult<(String)>> {
    if verbose {
        println!("Opening file {}", filepath);
    }

    let f = match File::open(filepath) {
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
    let mut results = Vec::new();
    
    if verbose {
        println!("Beginning to PING each address...");
    } 

    for line in buff.lines() {
        let data = &line.unwrap();
        let vec: Vec<&str> = data.split(':').collect();
        if verbose {
            println!("Sending PING for address {}", vec[0]);
        }

        results.push(send_ping(verbose, vec[0]));

        if verbose {
            println!("PING for address {} completed", vec[0]);
        }
    }

    if verbose {
        println!("Returning all PING responses to caller...");
    }

    results
}

// Run a PING on the given address
//
// NOTES ON RETURNING:
//      Returns an error if a timeout can't be set
//      Returns an error if IP address is not valid
//      Returns an error if socket cannot be established to IP Address
//      Returns an OK if PING is successful
//      Returns an OK if PING recieves no response
//
// Notice that the function returns an OK when the PING gets
// no response - this is because the socket connection is what
// throws an error, anything else is considered a successful connection
// but a failed PING.
//
// The String returned is what differentiates a successful or failed PING
fn send_ping(verbose: bool, ip: &str) -> PingResult<(String)> {
    let mut ping = Ping::new();
    try!(ping.set_timeout(5.0));    // timeout in seconds
    //try!(ping.add_host(ip));        // error if socket can't be created

    match ping.add_host(ip) {
        Ok(_)  => (),
        Err(e) => 
        { 
            let error_message = format!("Failure from {} - {}", ip, e);
            return Err(PingError::CustomError(error_message));
        },
    }

    let responses = try!(ping.send());
    let mut ret = "Uncaught error".to_owned();

    for resp in responses {
        if resp.dropped > 0 {
            ret = format!("Failure from {} - No response", resp.hostname).to_owned();
        } else {
            if !verbose {
                ret = format!("Success from {} - Response (address {}): latency {} ms",
                resp.hostname, resp.address, resp.latency_ms).to_owned();
            } else {
                ret = format!("Success from {} - Response {:?}", resp.hostname, resp).to_owned();
            }
        }
    }

    Ok(ret)
}
