use std::io;
use std::process::exit;

mod args;
mod version;
mod bytes;
mod utils;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = dotenv::var("APP").unwrap();
    let path = args::Args::load().app_path;
    let version = version::Version::get(&path).await;
    let binary = format!("{}/Contents/MacOS/{}", path, app);

    if !(version.0 <= 1 && version.1 <= 3 && version.2 <= 6) {
        printf!("This tool was tested up to version 1.1.5\nDo you want to continue for version {}.{}.{}? [y/n]: ", version.0, version.1, version.2);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        input = input.replace(" ", "").replace("\n", "").trim().parse().unwrap();
        if input != "y" {
            printf!("Ending program...\n");
            exit(0);
        }
    }
    printf!("Trying to crack ...\n");

    let mut b = bytes::Bytes::get_bytes(&binary);
    if let Some(address) = b.get_f_address() {
        printf!("    [*] Found address at: 0x{}\n", format!("{:x}", address).to_uppercase());
        let address = address + 10;

        if  b.0[address] == 0x85 && b.0[address + 2] == 0x74 {
            printf!("    [*] 0x{}: {} -> 89 \n", format!("{:x}", address).to_uppercase(),  format!("{:02x}", b.0[address]).to_uppercase());
            printf!("    [*] 0x{}: {} -> EB \n", format!("{:x}", address+2).to_uppercase(), format!("{:02x}", b.0[address + 2]).to_uppercase());

            b.0[address] = 0x89;
            b.0[address + 2] = 0xEB;

            b.write();
            printf!("    [*] Done!\n");
        } else if b.0[address] == 0x89 && b.0[address + 2] == 0xEB {
            eprintf!("    [x] Error: Already patched\n");
            exit(1);
        } else {
            eprintf!("    [x] Error: Found unexpected bytes: 0x{}, 0x{}\n", format!("{:02x}", b.0[address]).to_uppercase(), format!("{:02x}", b.0[address + 2]).to_uppercase());
            exit(1);
        }
    } else {
        eprintf!("    [x] Error: Failed to find pattern\n");
        exit(1);
    }
}
