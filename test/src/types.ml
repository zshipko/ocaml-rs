open Bigarray

external list_length: 'a list -> int = "list_length"

let%test "list length (empty)" = (list_length [] = 0)
let%test "list length (small)"= (list_length [1; 2; 3] = 3)
let%test "list length (big)" = (list_length (Array.make 10000 0 |> Array.to_list) = 10000)

external list_nil: unit -> 'a list = "list_nil"
external list_cons: 'a list -> 'a -> 'a list = "list_cons"

let%test "list nil" = (list_nil () = [])
let%test "list cons 1" = (list_cons (list_nil ()) 12.5 = [12.5])
let%test "list cons 2" = (list_cons (list_cons (list_nil ()) 12.5) 11.5 = [11.5; 12.5])

external array_make_range: int -> int -> int array = "array_make_range"
external array_make_range_f: int -> int -> float array = "array_make_range_f"
external array_replace: 'a array -> int -> 'a -> 'a option = "array_replace"

let%test "array make range 1" = (array_make_range 0 0 = [||])
let%test "array make range 2" = (array_make_range 0 10 = [|0; 1; 2; 3; 4; 5; 6; 7; 8; 9|])
let%test "array make range f" = (array_make_range_f 0 50_000 |> Array.length = 50_000)
let%test "array replace 1" = (
  let a = [| "A"; "B"; "C" |] in
  (array_replace a 1 "X" = (Some "B")) && (a.(1) = "X")
)


external array1_of_string: string -> (int, int8_unsigned_elt, c_layout) Array1.t = "array1_of_string"
external array1_new: int -> init:int -> (int, int8_unsigned_elt, c_layout) Array1.t = "array1_new"
external array1_from_rust_vec: unit -> (float, float32_elt, c_layout) Array1.t = "array1_from_rust_vec"

let%test "array1 of empty string" = (Array1.dim (array1_of_string "") = 0)
let%test "array1 of string 1" = (
  let a = array1_of_string "test" in
  Array1.dim a = 4 &&
  a.{0} = (int_of_char 't') &&
  a.{1} = (int_of_char 'e') &&
  a.{2} = (int_of_char 's') &&
  a.{3} = (int_of_char 't')
)
let%test "array1 new" = (
  let arr = array1_new 10 ~init:5 in
  let status = ref true in
  for i = 0 to 9 do
    status := !status && Array1.unsafe_get arr i = 5
  done;
  !status
)
let%test "array1 from rust vec" = (
  let a = array1_from_rust_vec () in
  a.{0} = 1. &&
  a.{1} = 2. &&
  a.{2} = 3. &&
  a.{3} = 4. &&
  a.{4} = 5.
)

type abstract_ptr

external alloc_abstract_pointer : float -> abstract_ptr = "alloc_abstract_pointer"
external abstract_pointer_value : abstract_ptr -> float = "abstract_pointer_value"
external abstract_pointer_free: abstract_ptr -> unit = "abstract_pointer_free"

let%test "abstract pointer" = (
  let a = alloc_abstract_pointer 1.5 in
  let f = abstract_pointer_value a in
  abstract_pointer_free a; f = 1.5
)
