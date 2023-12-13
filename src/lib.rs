pub mod error;
pub mod parser;

pub use error::Error;

use std::{fs::OpenOptions, io::Write, path::Path, process::Command};

pub fn add_to_log(log: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/log.txt")
        .unwrap();
    writeln!(file, "{}", log).unwrap();
}

pub fn run_scilla_fmt(path: &Path) -> Result<String, Error> {
    //docker run --rm -v contract.scilla:/tmp/input.scilla  -i zilliqa/scilla:v0.13.3 /scilla/0/bin/scilla-fmt --sexp --human-readable -d /tmp/input.scilla
    let volume = &format!(
        "{}:/tmp/input.scilla",
        path.canonicalize().unwrap().display()
    );

    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            volume,
            "-i",
            "zilliqa/scilla:v0.13.3",
            "/scilla/0/bin/scilla-fmt",
            "--sexp",
            "--human-readable",
            "-d",
            "/tmp/input.scilla",
        ])
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}

// pub fn generate(contracts_path: &Path, out_dir: &Path) -> Result<(), Error> {
//     let dest_path = Path::new(&out_dir).join("scilla_contracts.rs");

//     let mut file = std::fs::File::create(&dest_path)?;
//     for entry in std::fs::read_dir(contracts_path)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             match run_scilla_fmt(&path, out_dir) {
//                 Ok(sexp_path) => {
//                     let contract = parse_sexp(&sexp_path, path)?;
//                     add_to_log(&format!("Parsed: {:?}", contract));
//                     writeln!(file, "{}", contract)?;
//                 }
//                 Err(_) => {
//                     add_to_log(&format!("Failed to call scilla_fmt for {}", path.display()));
//                     continue;
//                 }
//             }
//         }
//     }

//     Ok(())
// }
