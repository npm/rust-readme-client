extern crate readme_client;

#[test]
fn fetch_scoped() {
    let resp = readme_client::fetch_latest("@ag_dubs/scoped-for-test");
    assert!(!resp.is_err(), "there should be no error");
    assert!(
        resp.unwrap()
            .contains("<!-- this HTML was generated using marky-markdown",)
    );
}

#[test]
fn fetch_latest_pass() {
    let resp = readme_client::fetch_latest("express");
    assert!(!resp.is_err(), "there should be no error");
    assert!(
        resp.unwrap()
            .contains("<!-- this HTML was generated using marky-markdown",)
    );
}

#[test]
fn fetch_latest_fail() {
    let resp = readme_client::fetch_latest("notapkgfartfart");
    assert!(resp.is_err(), "there should be an error");
}

#[test]
fn fetch_version_pass() {
    let resp = readme_client::fetch_version("express", "4.16.2");
    assert!(!resp.is_err(), "there should be no error");
    assert!(
        resp.unwrap()
            .contains("<!-- this HTML was generated using marky-markdown",)
    );
}

#[test]
fn fetch_version_fail() {
    let resp = readme_client::fetch_version("express", "4.10.2");
    assert!(resp.is_err(), "there should be an error");
}
