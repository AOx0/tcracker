use std::process::exit;
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader } };


#[derive(Debug)]
pub struct Version (pub usize, pub usize, pub usize );

impl Version {

    pub async fn get(path: &str) -> Version {
        let file = File::open(format!("{}/Contents/Info.plist", &path)).await.expect("Failed to open file");

        let reader = BufReader::new(file);

        let mut lines = reader.lines();

        let mut found = false;
        let v: String = loop {

            if let Some(line) = lines.next_line().await.expect("Failed to read file") {
                if found {
                    break line.to_string()
                        .replace("	<string>", "")
                        .replace("</string>", "");
                }

                if line.contains("CFBundleShortVersionString") { found = true; }
            } else {
                eprintln!("Failed to read Info.plist");
                exit(1);
            }

        };

        let mut numbers = vec![0; 3];
        let v: Vec<&str> = v.split(".").collect();

        for n in 0..v.len() {
           numbers[n] = v[n].parse::<usize>().unwrap_or_else(|_|{
               eprintln!("Error while parsing version");
               exit(1);
           });
        }

        Version (numbers[0], numbers[1], numbers[2] )
    }
}