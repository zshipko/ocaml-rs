use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(feature = "dune")]
mod dune;

#[cfg(feature = "dune")]
pub use dune::Dune;

struct Source {
    path: PathBuf,
    functions: Vec<String>,
    types: Vec<String>,
}

pub struct Sigs {
    base_dir: PathBuf,
    output: PathBuf,
    source: Vec<Source>,
}

fn strip_quotes(s: &str) -> &str {
    s.trim_start_matches('"').trim_end_matches('"')
}

fn snake_case(s: &str) -> String {
    let mut dest = String::new();
    for c in s.chars() {
        if !dest.is_empty() && c.is_uppercase() {
            dest.push('_');
        }
        dest.push(c.to_ascii_lowercase());
    }
    dest
}

fn handle(attrs: Vec<syn::Attribute>, mut f: impl FnMut(&str)) {
    for attr in attrs {
        let attr_name = attr
            .path
            .segments
            .iter()
            .map(|x| x.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        if attr_name == "sig" || attr_name == "ocaml::sig" {
            match &attr.tokens.into_iter().collect::<Vec<_>>()[..] {
                [proc_macro2::TokenTree::Group(g)] => {
                    let v = g.stream().into_iter().collect::<Vec<_>>();
                    if v.len() != 1 {
                        panic!("Invalid signature: {g}");
                    }
                    if let [proc_macro2::TokenTree::Literal(ref sig)] = v[..] {
                        let s = sig.to_string();
                        let ty = strip_quotes(&s);
                        f(ty)
                    }
                }
                [] => f(""),
                x => {
                    panic!("Invalid signature: {x:?}");
                }
            }
        }
    }
}

impl Sigs {
    pub fn new(p: impl AsRef<Path>) -> Sigs {
        let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let base_dir = root.join("src");
        Sigs {
            base_dir,
            output: p.as_ref().to_path_buf(),
            source: Vec::new(),
        }
    }

    pub fn with_source_dir(mut self, p: impl AsRef<Path>) -> Sigs {
        self.base_dir = p.as_ref().to_path_buf();
        self
    }

    fn parse(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let files = std::fs::read_dir(path)?;

        for file in files {
            let file = file?;
            if file.metadata()?.is_dir() {
                self.parse(&file.path())?;
                continue;
            }

            if Some(Some("rs")) != file.path().extension().map(|x| x.to_str()) {
                continue;
            }

            let path = file.path();
            let mut src = Source {
                path: path.clone(),
                functions: Vec::new(),
                types: Vec::new(),
            };
            let s = std::fs::read_to_string(&path)?;
            let t: syn::File = syn::parse_str(&s)
                .unwrap_or_else(|_| panic!("Unable to parse input file: {}", path.display()));

            for item in t.items {
                match item {
                    syn::Item::Fn(item_fn) => {
                        let name = &item_fn.sig.ident;
                        handle(item_fn.attrs, |ty| {
                            let def = if item_fn.sig.inputs.len() > 5 {
                                format!("external {name}: {ty} = \"{name}_bytecode\" \"{name}\"")
                            } else {
                                format!("external {name}: {ty} = \"{name}\"")
                            };
                            src.functions.push(def);
                        });
                    }
                    syn::Item::Struct(item) => {
                        let name = snake_case(&item.ident.to_string());
                        handle(item.attrs, |ty| {
                            let def = if ty.is_empty() {
                                format!("type {name}")
                            } else if !ty.trim_start().starts_with('{') {
                                format!("type {}{name}{} = {ty}", '{', '}')
                            } else {
                                format!("type {name} = {ty}")
                            };
                            src.types.push(def);
                        });
                    }
                    syn::Item::Enum(item) => {
                        let name = snake_case(&item.ident.to_string());
                        handle(item.attrs, |ty| {
                            let def = if ty.is_empty() {
                                format!("type {name}")
                            } else {
                                format!("type {name} = {ty}")
                            };
                            src.types.push(def);
                        });
                    }
                    syn::Item::Type(item) => {
                        let name = snake_case(&item.ident.to_string());
                        handle(item.attrs, |_ty| src.types.push(format!("type {name}")));
                    }
                    _ => (),
                }
            }

            if !src.functions.is_empty() || !src.types.is_empty() {
                self.source.push(src);
            }
        }

        Ok(())
    }

    fn generate_ml(&mut self) -> Result<(), std::io::Error> {
        let mut f = std::fs::File::create(&self.output).unwrap();

        writeln!(f, "(* Generated by ocaml-rs *)\n")?;
        writeln!(f, "open! Bigarray")?;

        for src in &self.source {
            writeln!(
                f,
                "\n(* file: {} *)\n",
                src.path.strip_prefix(&self.base_dir).unwrap().display()
            )?;

            for t in &src.types {
                writeln!(f, "{t}")?;
            }

            for func in &src.functions {
                writeln!(f, "{func}")?;
            }
        }

        Ok(())
    }

    fn generate_mli(&mut self) -> Result<(), std::io::Error> {
        let filename = self.output.with_extension("mli");
        let mut f = std::fs::File::create(&filename).unwrap();

        writeln!(f, "(* Generated by ocaml-rs *)\n")?;
        writeln!(f, "open! Bigarray")?;

        for src in &self.source {
            writeln!(
                f,
                "\n(* file: {} *)\n",
                src.path.strip_prefix(&self.base_dir).unwrap().display()
            )?;

            for t in &src.types {
                writeln!(f, "{t}")?;
            }

            for func in &src.functions {
                writeln!(f, "{func}")?;
            }
        }

        Ok(())
    }

    pub fn generate(mut self) -> Result<(), std::io::Error> {
        let dir = self.base_dir.clone();
        self.parse(&dir)?;

        self.source.sort_by(|a, b| a.path.cmp(&b.path));
        self.generate_ml()?;
        self.generate_mli()
    }
}
