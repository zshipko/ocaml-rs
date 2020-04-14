#[ocaml::func]
pub fn list_length(x: ocaml::List<ocaml::Value>) -> ocaml::Int {
    x.len() as ocaml::Int
}

#[ocaml::func]
pub fn list_nil() -> ocaml::List<ocaml::Value> {
    ocaml::List::empty()
}

#[ocaml::func]
pub fn list_cons(l: ocaml::List<ocaml::Value>, x: ocaml::Value) -> ocaml::List<ocaml::Value> {
    l.add(x)
}
