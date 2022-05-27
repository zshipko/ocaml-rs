use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Sigs {
    base_dir: PathBuf,
    output: PathBuf,
    functions: Vec<String>,
    types: Vec<String>,
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
            if let [proc_macro2::TokenTree::Group(g)] =
                &attr.tokens.clone().into_iter().collect::<Vec<_>>()[..]
            {
                let v = g.stream().into_iter().collect::<Vec<_>>();
                if let [proc_macro2::TokenTree::Literal(ref sig)] = v[..] {
                    let s = sig.to_string();
                    let ty = strip_quotes(&s);
                    f(ty)
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
            functions: Vec::new(),
            types: Vec::new(),
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
            }
            if let Some(Some("rs")) = file.path().extension().map(|x| x.to_str()) {
                let s = std::fs::read_to_string(file.path())?;
                let t: syn::File = syn::parse_str(&s).expect("Unable to parse file");

                for item in t.items {
                    match item {
                        syn::Item::Fn(item_fn) => {
                            let name = &item_fn.sig.ident;
                            handle(item_fn.attrs, |ty| {
                                let def = format!("external {name}: {ty} = \"{name}\"");
                                self.functions.push(def);
                            });
                        }
                        syn::Item::Struct(item) => {
                            let name = snake_case(&item.ident.to_string());
                            handle(item.attrs, |ty| {
                                let def = if ty.is_empty() {
                                    format!("type {name}")
                                } else {
                                    format!("type {name} = {ty}")
                                };
                                self.types.push(def);
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
                                self.types.push(def);
                            });
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate(mut self) -> Result<(), std::io::Error> {
        let dir = self.base_dir.clone();
        self.parse(&dir)?;

        let mut f = std::fs::File::create(self.output).unwrap();

        for t in self.types {
            writeln!(f, "{t}")?;
        }

        for func in self.functions {
            writeln!(f, "{func}")?;
        }

        Ok(())
    }
}
