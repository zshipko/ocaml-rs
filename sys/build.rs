#[allow(unused)]
use std::io::{BufRead, Write};

const CC_LIB_PREFIX: &str = "NATIVECCLIBS=";

const FLAT_FLOAT_ARRAY: &str = "FLAT_FLOAT_ARRAY=";

#[allow(unused)]
fn cc_libs(ocaml_path: &str) -> std::io::Result<Vec<String>> {
    let path = format!("{}/Makefile.config", ocaml_path);
    let f = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut output = Vec::new();

    for line in f.lines().map_while(Result::ok) {
        if line.starts_with(CC_LIB_PREFIX) {
            let line: Vec<_> = line.split('=').collect();
            let line = line[1].split(' ');
            output = line
                .filter_map(|x| {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.replace("-l", ""))
                    }
                })
                .collect();
        }
    }
    Ok(output)
}

#[allow(unused)]
fn flat_float_array(ocaml_path: &str) -> std::io::Result<String> {
    let path = format!("{}/Makefile.config", ocaml_path);
    let f = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut flat_float_array = String::new();

    for line in f.lines().map_while(Result::ok) {
        if line.starts_with(FLAT_FLOAT_ARRAY) {
            let line: Vec<_> = line.split('=').collect();
            flat_float_array = line[1].to_string();
        }
    }
    Ok(flat_float_array)
}

#[allow(unused)]
fn link(out_dir: std::path::PathBuf, ocamlopt: String, ocaml_path: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(out_dir.join("runtime.ml")).unwrap();
    write!(f, "")?;

    assert!(std::process::Command::new(ocamlopt)
        .args(["-output-complete-obj", "-o"])
        .arg(out_dir.join("rt.o"))
        .arg(out_dir.join("runtime.ml"))
        .status()?
        .success());

    let ar = std::env::var("AR").unwrap_or_else(|_| "ar".to_string());
    assert!(std::process::Command::new(ar)
        .arg("rcs")
        .arg(out_dir.join("libruntime.a"))
        .arg(out_dir.join("rt.o"))
        .status()?
        .success());

    for lib in cc_libs(ocaml_path)? {
        println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=runtime");

    println!("cargo:rustc-link-search={}", ocaml_path);

    println!("cargo:rustc-link-lib=static=asmrun");

    Ok(())
}

#[allow(unused)]
fn run() -> std::io::Result<()> {
    println!("cargo:rerun-if-file-changed=sys/ocaml-sys.c");
    println!("cargo:rerun-if-env-changed=OCAMLOPT");
    println!("cargo:rerun-if-env-changed=OCAML_VERSION");
    println!("cargo:rerun-if-env-changed=OCAML_WHERE_PATH");
    println!("cargo:rerun-if-env-changed=OPAM_SWITCH_PREFIX");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let ocaml_version = std::env::var("OCAML_VERSION");
    let ocaml_where_path = std::env::var("OCAML_WHERE_PATH");
    let ocamlopt = std::env::var("OCAMLOPT").unwrap_or_else(|_| "ocamlopt".to_string());

    let version: String;
    let ocaml_path: String;

    match (ocaml_version, ocaml_where_path) {
        (Ok(ver), Ok(path)) => {
            version = ver;
            ocaml_path = path;
        }
        _ => {
            version = std::str::from_utf8(
                std::process::Command::new(&ocamlopt)
                    .arg("-version")
                    .output()?
                    .stdout
                    .as_ref(),
            )
            .unwrap()
            .trim()
            .to_owned();
            ocaml_path = std::str::from_utf8(
                std::process::Command::new(&ocamlopt)
                    .arg("-where")
                    .output()?
                    .stdout
                    .as_ref(),
            )
            .unwrap()
            .trim()
            .to_owned();
        }
    }

    let bin_path = format!("{}/../../bin/ocamlopt", ocaml_path);

    let mut f = std::fs::File::create(out_dir.join("ocaml_compiler")).unwrap();
    std::io::Write::write_all(&mut f, bin_path.as_bytes()).unwrap();

    // Write OCaml version to file
    let mut f = std::fs::File::create(out_dir.join("ocaml_version")).unwrap();
    std::io::Write::write_all(&mut f, version.as_bytes()).unwrap();

    // Write OCaml path to file
    let mut f = std::fs::File::create(out_dir.join("ocaml_path")).unwrap();
    std::io::Write::write_all(&mut f, ocaml_path.as_bytes()).unwrap();

    let mut f = std::fs::File::create(out_dir.join("flat_float_array")).unwrap();
    std::io::Write::write_all(&mut f, flat_float_array(&ocaml_path)?.as_bytes()).unwrap();

    let split: Vec<&str> = version.split('.').collect();

    let major = split[0].parse::<usize>().unwrap();
    let minor = split[1].parse::<usize>().unwrap();

    if major >= 5 || cfg!(feature = "ocaml5") {
        println!("cargo:rustc-cfg=feature=\"ocaml5\"");
    }

    #[cfg(feature = "link")]
    link(out_dir, bin_path, ocaml_path.as_ref())?;

    // Build C bindings
    cc::Build::new()
        .static_flag(true)
        .file("src/ocaml-sys.c")
        .include(&ocaml_path)
        .compile("ocaml-sys");
    println!("cargo:rustc-link-lib=static=ocaml-sys");

    Ok(())
}

fn main() {
    #[cfg(not(feature = "without-ocamlopt"))]
    run().unwrap()
}
