use std::path::{Path, PathBuf};

pub struct Dune {
    root: PathBuf,
    library: PathBuf,
}

impl Dune {
    pub fn new(library: impl AsRef<Path>) -> Dune {
        Dune {
            root: PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()),
            library: library.as_ref().to_path_buf(),
        }
    }

    pub fn with_root(mut self, root: impl AsRef<Path>) -> Dune {
        self.root = root.as_ref().to_path_buf();
        self
    }

    fn run(&self) {
        let c = std::process::Command::new("dune")
            .current_dir(&self.root)
            .arg("build")
            .status()
            .unwrap();
        assert!(c.success());
    }

    pub fn build(self) {
        self.run();

        let path = self.root.join("_build").join("default").join(&self.library);

        let mut build = cc::Build::new();

        for file in std::fs::read_dir(&path).unwrap() {
            let file = file.unwrap();
            let path = file.path();
            if path.extension().map(|x| x.to_str().unwrap()) == Some("o") {
                build.object(&path);
            }
        }

        build.compile("ocaml");
    }
}
