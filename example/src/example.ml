open Bigarray

type testing =
    | First of float
    | Second of int

type something

external send_int : int -> int = "ml_send_int"
external send_two : int -> string -> unit = "ml_send_two"
external send_tuple : (int * int) -> int = "ml_send_tuple"
external send_int64 : int64 -> int64 = "ml_send_int64"
external new_tuple : unit -> (int * int * int) = "ml_new_tuple"
external new_array : unit -> int array = "ml_new_array"
external new_list : unit -> int list = "ml_new_list"
external testing_callback : int -> int -> unit = "ml_testing_callback"
external raise_not_found : unit -> unit = "ml_raise_not_found"
external send_float : float -> float = "ml_send_float"
external send_first_variant : unit -> testing = "ml_send_first_variant"
external custom_value : unit -> something = "ml_custom_value"
external array1 : unit -> (int, int8_unsigned_elt, c_layout) Array1.t = "ml_array1"
external string_test : string -> string = "ml_string_test"
external make_list: int -> int list = "ml_make_list"
