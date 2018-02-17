external send_int : int -> int = "ml_send_int"
external send_two : int -> string -> unit = "ml_send_two"
external send_tuple : (int * int) -> int = "ml_send_tuple"
external new_tuple : unit -> (int * int * int) = "ml_new_tuple"
external new_array : unit -> int array = "ml_new_array"
external new_list : unit -> int list = "ml_new_list"

let f x = x land 0x0000ffff


let _ =
  let string = "string thing" in
  let deadbeef = 0xdeadbeef in
  let res = send_int 0xb1b1eb0b in
  Printf.printf "send_int returned: 0x%x\n" res;
  flush stdout;
  send_two deadbeef string;
  send_two (f deadbeef) string;
  let res = send_tuple (1, 2) in
  Printf.printf "%d\n" res;
  let (a, b, c) = new_tuple () in
  Printf.printf "%d %d %d\n" a b c;
  let arr = new_array () in
  Array.iter (Printf.printf "%d\n") arr;
  let lst = new_list () in
  List.iter (Printf.printf "%d\n") lst
