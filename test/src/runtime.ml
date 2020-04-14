(* Unboxed / more than 5 arguments *)

external unboxed_float_avg: float -> float -> float = "unboxed_float_avg_bytecode" "unboxed_float_avg" [@@unboxed] [@@noalloc]
external more_than_five_params: float -> float -> float -> float -> float -> float -> float -> float = "more_than_five_params_bytecode" "more_than_five_params"


let%test "unboxed float avg 0" = unboxed_float_avg 0.0 0.0 = 0.0
let%test "unboxed float avg" = unboxed_float_avg 100.0 300.0 = 200.0
let%test "more than 5 params 0" = more_than_five_params 0.0 0.0 0.0 0.0 0.0 0.0 0.0 = 0.0
let%test "more than 5 params" = more_than_five_params 1.0 1.0 1.0 1.0 1.0 1.0 1.0 = 7.0

(* Exceptions *)

exception Exc of float

let () = Callback.register_exception "Exc" (Exc 0.0)

external raise_exc: float -> bool = "raise_exc"

let%test "raise exc" = try
  raise_exc 10.
with Exc x -> x = 10.

(* Hash variant *)
type hash_variant = [
  | `Abc of int
  | `Def of float
]

external hash_variant_abc: int -> hash_variant = "hash_variant_abc"
external hash_variant_def: float -> hash_variant = "hash_variant_def"

let%test "hash variant `Abc" = hash_variant_abc 123 = `Abc 123
let%test "hash variant `Def" = hash_variant_def 9. = `Def 9.
