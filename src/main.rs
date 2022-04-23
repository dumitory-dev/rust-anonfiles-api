use curl::easy::{Easy, Form};
use regex::Regex;
use std::error;
use std::str;

fn send_post_req(url: &str, form_data: Option<Form>) -> Result<Vec<u8>, Box<dyn error::Error>> {
    const STATUS_OK: u32 = 200;
    let mut easy = Easy::new();

    easy.url(url)?;
    easy.post(true)?;

    if let Some(form_data) = form_data {
        easy.httppost(form_data)?;
    }

    let mut response_data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            response_data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }

    let resp_code = easy.response_code().unwrap();

    if resp_code != STATUS_OK {
        return Err(format!("Error sending POST request! Response code: {}", resp_code).into());
    }

    Ok(response_data)
}

fn get_load_link_fron_resp(data: Vec<u8>) -> Result<String, String> {
    const SHORT_LINK: u8 = 2;
    let str_resp = match str::from_utf8(&data) {
        Ok(str_resp) => str_resp,
        Err(e) => {
            return Err(format!(
                "Anonfile: Invalid UTF-8 sequence from response: {}",
                e
            ))
        }
    };

    let regex = Regex::new(r#"("short":")(\S+)("},"metadata")"#).unwrap();

    if let Some(captures) = regex.captures(str_resp) {
        if let Some(capture) = captures.get(SHORT_LINK.into()) {
            return Ok(capture.as_str().to_string());
        }
    }

    Err(format!("Anonfile: Failed to parse json response!"))
}

pub fn load_file(
    path_to_file: impl AsRef<std::path::Path>,
) -> Result<String, Box<dyn error::Error>> {
    let url = "https://api.anonfiles.com/upload";
    let mut form_data = Form::new();

    if let Err(err) = form_data.part("file").file(&path_to_file).add() {
        return Err(format!("Anonfile: Error uploading file! {}", err.description()).into());
    }

    let resp_data = send_post_req(url, Some(form_data))?;
    get_load_link_fron_resp(resp_data)
}

fn main() {
    println!("Hello, world!");
}