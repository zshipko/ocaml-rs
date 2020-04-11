open Bigarray

type testing =
    | First of float
    | Second of int

type something

type my_record = {
  foo: string;
  bar: float;
}

type hash_variant = [
  | `Abc of int
  | `Def of float
]

type custom_example

exception Exc of int

let () = Callback.register_exception "Exc" (Exc 0)

external send_int : int -> int = "ml_send_int"
external send_two : int -> string -> unit = "ml_send_two"
external send_tuple : (int * int) -> int = "ml_send_tuple"
external send_int64 : int64 -> int64 = "ml_send_int64"
external new_tuple : int -> (int * int * int) = "ml_new_tuple"
external new_array : int -> int array = "ml_new_array"
external new_list : int -> int list = "ml_new_list"
external testing_callback : int -> int -> unit = "ml_testing_callback"
external raise_not_found : unit -> unit = "ml_raise_not_found"
external raise_failure : unit -> unit = "ml_raise_failure"
external raise_exc: int -> unit = "ml_raise_exc"
external send_float : float -> float = "ml_send_float"
external send_first_variant : unit -> testing = "ml_send_first_variant"
external final_value: unit -> something = "ml_final_value"
external array1 : int -> (int, int8_unsigned_elt, c_layout) Array1.t = "ml_array1"
external array2: string -> (char, int8_unsigned_elt, c_layout) Array1.t = "ml_array2"
external string_test : string -> string = "ml_string_test"
external make_list: int -> int list = "ml_make_list"
external make_array: int -> int array = "ml_make_array"
external call: ('a -> 'b) -> 'a -> 'b = "ml_call"
external format_my_record: my_record -> string = "ml_format_my_record"
external unboxed_float: float -> float -> float = "ml_unboxed_float_bytecode" "ml_unboxed_float" [@@unboxed] [@@noalloc]
external more_than_five_params: float -> float -> float -> float -> float -> float -> float = "ml_more_than_five_params_bytecode" "ml_more_than_five_params"
external hash_variant: unit -> hash_variant = "ml_hash_variant"
external custom_value: int -> custom_example = "ml_custom_value"
external custom_value_int: custom_example -> int = "ml_custom_value_int"
external list_hd_len: 'a list -> 'a option * int = "ml_list_hd_len"
