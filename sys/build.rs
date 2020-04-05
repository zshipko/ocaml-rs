fn run() -> std::io::Result<()> {
    let cmd = std::env::var("OCAML").unwrap_or("ocaml".to_string());
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let output = std::process::Command::new(cmd)
        .arg("version.ml")
        .arg(&out_dir)
        .output()?;

    let mut f = std::fs::File::create(out_dir.join("version")).unwrap();
    std::io::Write::write_all(&mut f, &output.stdout).unwrap();

    let output = String::from_utf8(output.stdout).unwrap();
    let split: Vec<&str> = output.split('.').collect();

    let major = split[0].parse::<usize>().unwrap();
    let minor = split[1].parse::<usize>().unwrap();

    if major >= 4 && minor >= 10 {
        println!("cargo:rustc-cfg=caml_state");
    }

    Ok(())
}

fn main() {
    let _ = run();
}
