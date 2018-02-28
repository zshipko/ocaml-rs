type testing =
    | First of float
    | Second of int

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

let f x = x land 0x0000ffff

let print_testing a b =
    Printf.printf "testing: %d %d\n" a b

let _ =
    Callback.register "print_testing" print_testing

let _ =

    (* send_int *)
    let string = "string thing" in
    let deadbeef = 0xdeadbeef in
    let res = send_int 0xb1b1eb0b in
    Printf.printf "send_int returned: 0x%x\n" res;
    flush stdout;
    assert (res = 0xbeef);

    (* send_two *)
    send_two deadbeef string;
    send_two (f deadbeef) string;

    (* send_tuple *)
    let res = send_tuple (1, 2) in
    Printf.printf "%d\n" res;
    assert (res = 3);

    (* send_int64 *)
    let res = send_int64 15L in
    Printf.printf "%Ld\n" res;
    assert (res = 25L);

    (* new_tuple *)
    let (a, b, c) = new_tuple () in
    Printf.printf "%d %d %d\n" a b c;
    assert (a = 0 && b = 1 && c = 2);

    (* new_array *)
    let arr = new_array () in
    Array.iter (Printf.printf "%d\n") arr;
    assert (arr = [| 0; 1; 2; 3; 4 |]);

    (* new list *)
    let lst = new_list () in
    List.iter (Printf.printf "%d\n") lst;
    assert (lst = [0; 1; 2; 3; 4]);

    (* testing_callback *)
    testing_callback 5 10;

    (* raise Not_found *)
    try raise_not_found ()
    with Not_found -> print_endline "Got Not_found";

    (* send float *)
    let f = send_float 2.5 in
    Printf.printf "send_float: %f\n" f;
    flush stdout;
    assert (f = 5.0);

    (* send first variant *)
    assert (send_first_variant () = First (2.0));

    Gc.full_major ();
    Gc.minor()
