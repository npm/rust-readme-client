extern crate readme_client;

#[test]
fn fetch_latest() {
  let resp = readme_client::fetch_latest("express");
  assert!(!resp.is_err(), "there should be no error");
  match resp {
    Ok(resp) => assert!(resp.contains("express")),
    Err(e) => println!("error: {:?}", e),
  }
}

#[test]
fn fetch_version() {
  let resp = readme_client::fetch_version("express", "3.8.1");
  assert!(!resp.is_err(), "there should be no error");
  match resp {
    Ok(resp) => {
      assert!(resp.contains("express"));
      println!("{:?}", resp);
    },
    Err(e) => println!("error: {:?}", e),
  }
}
