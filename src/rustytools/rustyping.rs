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
use utilities::helpers::{read_file, process_subnet};

use std::thread;
use std::sync::mpsc;

// PING a single IP Address
pub fn ping_ip(verbose: bool, ip: &str) -> PingResult<(String)> {
    send_ping(verbose, ip)
}

// PING an entire subnet of IP Addresses
// Currently limited to Class C subnets ONLY
//      This means your mask bits can be 24 - 30
pub fn ping_subnet(verbose: bool, subnet: &str) -> Vec<PingResult<(String)>> {
    let mut results = Vec::new();
    let (tx, rx) = mpsc::channel();

    let subc = subnet.clone().to_owned();
    thread::spawn(move || {
        process_subnet(verbose, subc, tx);
    });

    for rec in rx {
        if verbose {
            println!("Sending PING for address {}", rec);
        }

        results.push(send_ping(verbose, &rec));

        if verbose {
            println!("PING for address {} completed", rec);
        }
    }

    if verbose {
        println!("Returning all PING responses to caller...");
    }

    results
}

// PING every IP Address in a file
pub fn ping_file(verbose: bool, filepath: &str) -> Vec<PingResult<(String)>> {
    let mut results = Vec::new();
    let (tx, rx) = mpsc::channel();

    let fpc = filepath.clone().to_owned();
    thread::spawn(move || {
        read_file(verbose, fpc, tx);
    });

    if verbose {
        println!("Beginning to PING each address...");
    } 

    for rec in rx { 
        let vec: Vec<&str> = rec.split(':').collect();
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
