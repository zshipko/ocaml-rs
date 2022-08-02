open Rust
open Bigarray

let%test "list length (empty)" = Util.check_leaks (fun () -> list_length [] = 0)
let%test "list length (small)"= Util.check_leaks (fun () -> list_length [1; 2; 3] = 3)
let%test "list length (big)" = Util.check_leaks (fun ()-> list_length (Array.make 10000 0 |> Array.to_list) = 10000)

let%test "list nil" = Util.check_leaks (fun () -> list_nil () = [])
let%test "list cons 1" = Util.check_leaks (fun () -> list_cons (list_nil ()) 12.5 = [12.5])
let%test "list cons 2" = Util.check_leaks (fun () -> let a = list_cons (list_cons (list_nil ()) 12.5) 11.5 in Util.gc (); a = [11.5; 12.5])

let%test "array make range 1" = Util.check_leaks (fun () -> array_make_range 0 0 = [||])
let%test "array make range 2" = Util.check_leaks (fun () -> let a = array_make_range 0 10 in Util.gc (); a = [|0; 1; 2; 3; 4; 5; 6; 7; 8; 9|])
let%test "array make range f" = Util.check_leaks (fun () -> let a = array_make_range_f 0 50_000 in Util.gc (); Array.length a = 50_000)
let%test "array replace 1" = Util.check_leaks (fun () ->
  let a = [| "A"; "B"; "C" |] in
  (array_replace a 1 "X" = (Some "B")) && (a.(1) = "X")
)

let%test "array1 of empty string" = Util.check_leaks (fun () -> Array1.dim (array1_of_string "") = 0)
let%test "array1 of string 1" = Util.check_leaks (fun () ->
  let a = array1_of_string "test" in
  Util.gc ();
  Array1.dim a = 4 &&
  a.{0} = (int_of_char 't') &&
  a.{1} = (int_of_char 'e') &&
  a.{2} = (int_of_char 's') &&
  a.{3} = (int_of_char 't')
)
let%test "array1 new" = Util.check_leaks (fun () ->
  let arr = array1_new 10 ~init:5 in
  Util.gc ();
  let status = ref true in
  for i = 0 to 9 do
    status := !status && Array1.unsafe_get arr i = 5
  done;
  !status
)
let%test "array1 from rust vec" = Util.check_leaks (fun () ->
  let a = array1_from_rust_vec () in
  Util.gc ();
  a.{0} = 1. &&
  a.{1} = 2. &&
  a.{2} = 3. &&
  a.{3} = 4. &&
  a.{4} = 5.
)

type array2_t = (float, float32_elt, c_layout) Array2.t

external make_array2: int -> int -> array2_t = "make_array2"
external array2_format: array2_t -> string = "array2_format"
external array2_set: array2_t -> int -> int -> float -> unit = "array2_set"
external array2_get: array2_t -> int -> int -> float = "array2_get"

let test_array2_checked dim1 dim2 = (
  let arr = make_array2 dim1 dim2 in
  let rec check x y v =
    if not v || x == dim1 then v else
      if y == dim2 then
        check (x + 1) 0 v
      else
        let value = float_of_int (x * y) in
        array2_set arr x y value;
        check x (y + 1) (array2_get arr x y = value && arr.{x, y} = value)
  in
  arr, check 0 0 true
)

let%test "array2" = Util.check_leaks (fun () ->
  let dim1 = 9000 and dim2 = 800 in
  let _, check = test_array2_checked dim1 dim2 in
  check
)

let%test "array2_format" = Util.check_leaks (fun () ->
  let dim1 = 3 and dim2 = 3 in
  let arr, check = test_array2_checked dim1 dim2 in
  let () = Util.gc () in
  check && (array2_format arr) = "[[0, 0, 0], [0, 1, 2], [0, 2, 4]]"
)

type abstract_ptr

external alloc_abstract_pointer : float -> abstract_ptr = "alloc_abstract_pointer"
external abstract_pointer_value : abstract_ptr -> float = "abstract_pointer_value"
external abstract_pointer_free: abstract_ptr -> unit = "abstract_pointer_free"

let%test "abstract pointer" = Util.check_leaks (fun () ->
  let a = alloc_abstract_pointer 1.5 in
  Util.gc ();
  let f = abstract_pointer_value a in
  Util.gc ();
  abstract_pointer_free a; f = 1.5
)

let%test "seq sum" = Util.check_leaks (fun () ->
  let l = List.init 100 (fun x -> x) in
  let s = List.to_seq l in
  let sum = seq_sum s in
  let sum' = List.fold_left ( + ) 0 l in
  sum = sum'
)
