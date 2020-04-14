#[ocaml::func]
pub fn list_length(x: ocaml::List<ocaml::Value>) -> ocaml::Int {
    x.len() as ocaml::Int
}
