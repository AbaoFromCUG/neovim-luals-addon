use std::process::Command;
mod api_info;
use api_info::ApiInfo;

fn main() {
    let output = Command::new("nvim")
        .arg("--api-info")
        .output()
        .expect("Failed to execute neovim");

    // print!("{:?}", output.stdout.as_slice());
    let api_info: ApiInfo =
        rmp_serde::from_slice(output.stdout.as_slice()).expect("Failed to deserialize msgpack");
    println!("{:?}", api_info);
}
