use downloader::Downloader;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "Abbreviation": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

// Run example with: cargo run --example tui_basic --features tui
fn download() {
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("data/"))
        .parallel_requests(1)
        .build()
        .unwrap();

    // Download with an explicit filename
    let dl = downloader::Download::new(
        "https://raw.githubusercontent.com/d-edge/foss-acronyms/main/data/acronyms.json",
    );
    let result = downloader.download(&[dl]).unwrap();
    for r in result {
        match r {
            Err(e) => print!("Error occurred! {}", e.to_string()),
            Ok(s) => {
                print!("Success: {}", &s);
            }
        };
    }
}

fn main() {
    if !std::path::Path::new("data/acronyms.json").exists() {
        download();
    }
    let res = typed_example();
    print!("{:?}", res);
}
