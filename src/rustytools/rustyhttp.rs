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

use super::super::{HttpError, HttpResult};
use reqwest;

// Send HTTP request to single IP Address
pub fn http_ip(verbose: bool, ip: &str) -> HttpResult<(String)> {
    send_http(verbose, ip)
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
