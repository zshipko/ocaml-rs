
open Rust

(* Unboxed *)
external unboxed_float_avg: float -> float -> float = "unboxed_float_avg_bytecode" "unboxed_float_avg" [@@unboxed] [@@noalloc]

let%test "unboxed float avg 0" = Util.check_leaks (fun () -> let a = unboxed_float_avg 0.0 0.0 in Util.gc (); a = 0.0)
let%test "unboxed float avg" = Util.check_leaks (fun () -> unboxed_float_avg 100.0 300.0 = 200.0)
let%test "more than 5 params 0" = Util.check_leaks (fun () -> let a = more_than_five_params 0.0 0.0 0.0 0.0 0.0 0.0 0.0 in Util.gc (); a =  0.0)
let%test "more than 5 params" = Util.check_leaks (fun () -> let a = more_than_five_params 1.0 1.0 1.0 1.0 1.0 1.0 1.0  in Util.gc (); a = 7.0)
let%test "too many arguments" = Util.check_leaks (fun () -> (
  mutable_parameter_with_more_than_five_arguments true true 0L 0L None None;
  true)
)

(* Exceptions *)

exception Exc of float

exception Rust of string

let () = Callback.register_exception "Exc" (Exc 0.0)

let%test "raise exc" = try
  raise_exc 10.; true
with Exc x -> x = 10.

let%test "raise failure" = try
  raise_failure (); true
with Failure e -> let () = Util.gc () in e = "An error"

(* Hash variant *)
type hash_variant = [
  | `Abc of int
  | `Def of float
]

let%test "hash variant `Abc" = Util.check_leaks (fun () -> let a = hash_variant_abc 123 in Util.gc (); a = `Abc 123)
let%test "hash variant `Def" = Util.check_leaks (fun () -> let a = hash_variant_def 9. in Util.gc (); a = `Def 9.)

let%test "test panic" = Util.check_leaks (fun () -> try
  let _ = test_panic () in
  false
with
  | Failure s -> begin
    Util.gc ();
    s = "XXX"
  end
  | _ -> false)


let () = Callback.register "call_named" (fun x -> x *. 2.)

let%test "test call named" = Util.check_leaks (fun () ->
  let x = test_call_named 2.0 in
  Util.gc ();
  x = 4.0
)



let%test "exn" = Util.check_leaks (fun () -> (
  let str = exn_to_string (Invalid_argument "test") in
  str = "Invalid_argument(\"test\")"
))


let%test "GC" =
  Random.init 0;
  let test f =
    let i = Random.int 1337 in
    let s = string_of_int i in
    f();
    s = string_of_int i
  in
  List.for_all test [gc_minor; gc_major; gc_full_major; gc_compact]
