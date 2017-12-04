extern crate npm_readme_client;

#[cfg(test)]
extern crate mocktopus;

mod mock;

#[cfg(test)]
use mocktopus::mocking::*;

//use npm_readme_client::error::Error;

#[test]
fn fetch_scoped() {
    // TODO: figure out why this doesn't actually override the function
    npm_readme_client::fetch_latest.mock_safe(|_pkg_name| MockResult::Return(mock::readme()));
    let resp = npm_readme_client::fetch_latest(String::from("@ag_dubs/scoped-for-test"));
    //match resp {
    //    Ok(ref rsp) => println!("Received a fragment: {}", rsp),
    //    Err(Error::Io(ref _err)) => println!("I/O Error"),
    //    Err(Error::Reqwest(ref err)) => println!("Reqwest Error: {}", err),
    //    Err(Error::Response(ref _err)) => println!("Response Error"),
    //};
    assert!(!resp.is_err(), "there should be no error");
    assert!(resp.unwrap().contains(
        "<!-- this HTML was generated using marky-markdown",
    ));
}

#[test]
fn fetch_latest_pass() {
    let resp = npm_readme_client::fetch_latest(String::from("express"));
    assert!(!resp.is_err(), "there should be no error");
    assert!(resp.unwrap().contains(
        "<!-- this HTML was generated using marky-markdown",
    ));
}

#[test]
fn fetch_latest_fail() {
    let resp = npm_readme_client::fetch_latest(String::from("notapkgfartfart"));
    assert!(resp.is_err(), "there should be an error");
}

#[test]
fn fetch_version_pass() {
    let resp = npm_readme_client::fetch_version(String::from("express"), String::from("4.16.2"));
    assert!(!resp.is_err(), "there should be no error");
    assert!(resp.unwrap().contains(
        "<!-- this HTML was generated using marky-markdown",
    ));
}

#[test]
fn fetch_version_fail() {
    let resp = npm_readme_client::fetch_version(String::from("express"), String::from("4.10.2"));
    assert!(resp.is_err(), "there should be an error");
}
