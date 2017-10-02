/* rustworking-core
 *
 * Rust crate to handle network tasks for
 * system administration.
 *
 * Set of functions to hanlde testing IP Addresses
 * using HTTP.
 *
 * Author: Tim Monfette
 * Version: 0.1.0
 */

use super::super::{HttpError, HttpResult};
use utilities::helpers::{read_file, process_subnet};
use reqwest;

use std::thread;
use std::sync::mpsc;

// Send HTTP request to single IP Address
pub fn http_ip(verbose: bool, ip: &str) -> HttpResult<(String)> {
    send_http(verbose, ip)
}

// Send HTTP request to  an entire subnet of IP Addresses
// Currently limited to Class C subnets ONLY
//      This means your mask bits can be 24 - 30
pub fn http_subnet(verbose: bool, subnet: &str) -> Vec<HttpResult<(String)>> {
    let mut results = Vec::new();
    let (tx, rx) = mpsc::channel();

    let subc = subnet.clone().to_owned();
    thread::spawn(move || {
        process_subnet(verbose, subc, tx);
    });

    for rec in rx {
        if verbose {
            println!("Sending HTTP request for address {}", rec);
        }

        results.push(send_http(verbose, &rec));

        if verbose {
            println!("HTTP request for address {} completed", rec);
        }
    }

    if verbose {
        println!("Returning all HTTP responses to caller...");
    }

    results
}

// Send HTTP request to every address in a file
pub fn http_file(verbose: bool, filepath: &str) -> Vec<HttpResult<(String)>> {
    let mut results = Vec::new();
    let (tx, rx) = mpsc::channel();

    let fpc = filepath.clone().to_owned();
    thread::spawn(move || {
        read_file(verbose, fpc, tx);
    });

    if verbose {
        println!("Beginning to test HTTP on each address...");
    }

    for rec in rx {
        let vec: Vec<&str> = rec.split(':').collect();
        if verbose {
            println!("Sending HTTP request for address {}", vec[0]);
        }

        results.push(send_http(verbose, vec[0]));

        if verbose {
            println!("HTTP request for address {} completed", vec[0]);
        }
    }

    if verbose {
        println!("Returning all HTTP responses to caller...");
    }

    results
}

// Process an HTTP request on an IP Address
//
// NOTES ON RETURNING:
//      Returns an error is returned if a connection cannot be established.
//      Returns an error is returned if the request failed to send.
//      Returns an OK if a resonse is returned from the IP.
//      Returns an OK even if the response code isn't 200 - as long as one is gotten.
//
// Note that this only uses "http" right now.
fn send_http(verbose: bool, ip: &str) -> HttpResult<(String)> {
    let url = format!("http://{}", ip);

    match reqwest::get(&url) {
        Ok(resp)  =>
        {
            let ret: String;
            if verbose { 
                ret = format!("Success from {} - Status Code {} - Headers {:?}", ip, resp.status(), resp.headers()).to_owned(); 
            } else {
                ret = format!("Success from {} - Status Code {}", ip, resp.status()).to_owned();
            }
            return Ok(ret);
        },
        Err(e) =>
        {
            let error_message = format!("Failure from {} - {}", ip, e);
            return Err(HttpError::ErrorMessage(error_message));
        },
    } 
}
