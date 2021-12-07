use error_chain::error_chain;
use reqwest::header::USER_AGENT;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use tempfile::Builder;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

pub async fn donwload_puzzle(day: u8, n: u8) -> Result<String> {
    let s = format!("./inputs/day_{}", day);
    fs::create_dir_all("./inputs/");
    // println!("{}", s.clone());
    let exists = Path::new(&s).exists();

    let cookie = include_str!("../cookie.txt");
    println!("using cookie: '{}'", cookie);
    if !exists {
        println!("file {} does not exist, downloading: ", exists);
        let target = format!("https://adventofcode.com/2021/day/{}/input", day);
        let client = reqwest::Client::new();
        let session = format!("session={cookie}", cookie = cookie);
        let response = client.get(target).header("cookie", session).send().await?;

        let mut dest: File = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            println!("file to download: '{}'", fname);
            let fname = Path::new(&s);
            println!("will be located under: '{:?}'", fname);
            File::create(fname)?
        };
        let content = response.text().await?;
        copy(&mut content.as_bytes(), &mut dest)?;
    }
    Ok(fs::read_to_string(s)?)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        donwload_puzzle(1, 1);
    }
}
