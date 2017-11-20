extern crate readme_client;

#[test]
fn fetch_latest() {
  let resp = readme_client::fetch_latest("express");
  assert!(!resp.is_err(), "there should be no error");
  assert!(resp.unwrap().contains("express"))
}

#[test]
fn fetch_version() {
  let resp = readme_client::fetch_version("express", "4.10.2");
  assert!(!resp.is_err(), "there should be no error");
  assert!(resp.unwrap().contains("express"))
}
