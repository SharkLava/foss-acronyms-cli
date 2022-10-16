use downloader::Downloader;

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
}
