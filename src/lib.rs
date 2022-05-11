use std::fs;

use curl::easy::Easy;

pub fn get_download_link(item: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let mut curl = Easy::new();
    curl.url(&format!(
        "https://minecraft.fandom.com/api.php?action=imageserving&wisTitle={}&format=json",
        &item
    ))?;
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform().unwrap();
    }
    curl.perform()?;
    let json = json::parse(&String::from_utf8(data)?)?;
    if !json["image"]["imageserving"].is_null() {
        Ok(json["image"]["imageserving"].to_string())
    } else {
        #[derive(Debug)]
        struct NotFound(String);

        impl std::fmt::Display for NotFound {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::error::Error for NotFound {}
        
        Err(Box::new(NotFound(json["error"]["info"].to_string())))
    }
}

pub fn download_file(url: &str, path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut curl = Easy::new();
    curl.url(url)?;
    curl.write_function(move |data| {
        fs::write(&path, data).unwrap();
        Ok(data.len())
    })?;
    curl.perform()?;
    Ok(())
}
