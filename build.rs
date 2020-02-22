fn main() {
    let cmd = std::env::var("OCAML").unwrap_or("ocaml".to_string());
    let output = std::process::Command::new(cmd)
        .arg("version.ml")
        .arg(std::env::var("OUT_DIR").unwrap())
        .output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    let split: Vec<&str> = output.split('.').collect();

    let major = split[0].parse::<usize>().unwrap();
    let minor = split[1].parse::<usize>().unwrap();

    if major >= 4 && minor >= 10 {
        println!("cargo:rustc-cfg=caml_state");
    }
}
