(* Unboxed / more than 5 arguments *)

external unboxed_float_avg: float -> float -> float = "unboxed_float_avg_bytecode" "unboxed_float_avg" [@@unboxed] [@@noalloc]
external more_than_five_params: float -> float -> float -> float -> float -> float -> float -> float = "more_than_five_params_bytecode" "more_than_five_params"

external mutable_parameter_with_more_than_five_arguments_: bool -> bool -> Int64.t -> Int64.t -> Int64.t option -> Int32.t option -> unit = "mutable_parameter_with_more_than_five_arguments_bytecode" "mutable_parameter_with_more_than_five_arguments"

let%test "unboxed float avg 0" = Util.check_leaks (fun () -> let a = unboxed_float_avg 0.0 0.0 in Util.gc (); a = 0.0)
let%test "unboxed float avg" = Util.check_leaks (fun () -> unboxed_float_avg 100.0 300.0 = 200.0)
let%test "more than 5 params 0" = Util.check_leaks (fun () -> let a = more_than_five_params 0.0 0.0 0.0 0.0 0.0 0.0 0.0 in Util.gc (); a =  0.0)
let%test "more than 5 params" = Util.check_leaks (fun () -> let a = more_than_five_params 1.0 1.0 1.0 1.0 1.0 1.0 1.0  in Util.gc (); a = 7.0)
let%test "too many arguments" = Util.check_leaks (fun () -> (
  mutable_parameter_with_more_than_five_arguments_ true true 0L 0L None None;
  true)
)

(* Exceptions *)

exception Exc of float

exception Rust of string

let () = Callback.register_exception "Exc" (Exc 0.0)

external raise_exc: float -> bool = "raise_exc"
external raise_failure: unit -> bool = "raise_failure"

let%test "raise exc" = try
  raise_exc 10.
with Exc x -> x = 10.

let%test "raise failure" = try
  raise_failure ()
with Failure e -> let () = Util.gc () in e = "An error"

(* Hash variant *)
type hash_variant = [
  | `Abc of int
  | `Def of float
]

external hash_variant_abc: int -> hash_variant = "hash_variant_abc"
external hash_variant_def: float -> hash_variant = "hash_variant_def"

let%test "hash variant `Abc" = Util.check_leaks (fun () -> let a = hash_variant_abc 123 in Util.gc (); a = `Abc 123)
let%test "hash variant `Def" = Util.check_leaks (fun () -> let a = hash_variant_def 9. in Util.gc (); a = `Def 9.)

external test_panic: unit -> int = "test_panic"

let%test "test panic" = Util.check_leaks (fun () -> try
  let _ = test_panic () in
  false
with
  | Failure s -> begin
    Util.gc ();
    s = "XXX"
  end
  | _ -> false)

let %test "test custom panic exception" = Util.check_leaks (fun () -> try
  let () = Callback.register_exception "Rust_exception" (Rust "") in
  let _ = test_panic () in
  false
with
  | Rust s -> s = "XXX"
  | _ -> false)

let () = Callback.register "call_named" (fun x -> x *. 2.)
external test_call_named : float -> float = "test_call_named"

let%test "test call named" = Util.check_leaks (fun () ->
  let x = test_call_named 2.0 in
  Util.gc ();
  x = 4.0
)

external func : unit -> unit = "bench_func"
external native_func : unit -> unit = "bench_native_func"

external exn_to_string: exn -> string = "exn_to_string"

let%test "exn" = Util.check_leaks (fun () -> (
  let str = exn_to_string (Invalid_argument "test") in
  str = "Invalid_argument(\"test\")"
))
 
external gc_minor: unit -> unit = "gc_minor"
external gc_major: unit -> unit = "gc_major"
external gc_full_major: unit -> unit = "gc_full_major"
external gc_compact: unit -> unit = "gc_compact"

let%test "GC" =
  Random.init 0;
  let test f =
    let i = Random.int 1337 in
    let s = Int.to_string i in
    f();
    s = Int.to_string i
  in
  List.for_all test [gc_minor; gc_major; gc_full_major; gc_compact]
