use std::env::args;
use std::error::Error;

use gp8ctl::bluetooth::*;
use gp8ctl::wifi::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args().collect::<Vec<String>>();

    if args.len() > 1 {
        let cmd = args
            .into_iter()
            .nth(1)
            .unwrap();

        match cmd.as_str() {
            "--show-camera-info" => show_camera_info().await?,
            "--list-media" => list_media()?,
            _ => println!("command not available")
        }
    }

    Ok(())
}