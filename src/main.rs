use std::process::exit;

mod args;
mod version;
mod bytes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = dotenv::var("APP").unwrap();
    let path = args::Args::load().app_path;
    let version = version::Version::get(&path).await;
    let binary = format!("{}/Contents/MacOS/{}", path, app);

    if version.0 <= 1 && version.1 <= 0 && version.2 <= 5 {
        let mut b = bytes::Bytes::get_bytes(&binary);
        if let Some(address) = b.get_f_address() {
            println!("Found address at: 0x{}", format!("{:x}", address).to_uppercase());
            let address = address + 27;

            if  b.0[address] == 0x85 && b.0[address + 2] == 0x74 {
                println!("{}: {:02x} -> 89 ", address,  b.0[address]);
                println!("{}: {:02x} -> eb", address+2, b.0[address + 2]);

                b.0[address] = 0x89;
                b.0[address + 2] = 0xEB;

                b.write();
            } else if b.0[address] == 0x89 && b.0[address + 2] == 0xEB {
                eprintln!("Already patched");
                exit(1);
            } else {
                eprintln!("Found unexpected bytes: 0x{:x}, 0x{:x}", b.0[address], b.0[address + 2]);
                exit(1);
            }
        } else {
            eprintln!("Failed to find pattern");
            exit(1);
        }
    }
}
