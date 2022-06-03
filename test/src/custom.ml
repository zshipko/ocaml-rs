open Rust

let%test "testing compare 1" = Util.check_leaks (fun () -> testing_alloc 0L <> testing_alloc 1L)
let%test "testing compare 2" = Util.check_leaks (fun () -> testing_alloc 99L = testing_alloc 99L)
let%test "testing set c" = Util.check_leaks (fun () -> (
  let t = testing_alloc 1L in
  let () = Util.gc () in
  testing_set_a t 3.14;
  Util.gc ();
  testing_set_c t "FOOBAR";
  Util.gc ();
  let (a, b, c) = testing_get_values t in
  Util.gc ();
  a = 3.14 && b = 1L && c = "FOOBAR"
))

let%test "testing callback 1" = Util.check_leaks (fun () -> (
  let c = testing_callback_alloc (fun x -> float_of_int x *. 2.) in
  Util.gc ();
  testing_callback_call c 1 = 2.0)
)

let%test "testing callback 2" = Util.check_leaks (fun () -> (
  let c = testing_callback_alloc (fun x ->
    let () = Unix.sleep 2 in
    sin (float_of_int x)) in
  Util.gc ();
  testing_callback_call c 5 = sin 5.0)
)

let%test "testing abstract" = Util.check_leaks (fun () -> (
  let a = open_in "./custom.ml" in
  let len = in_channel_length a in
  let s = really_input_string a len in
  let () = close_in a in
  assert (String.length s = len);
  let f = file_open "./custom.ml" in
  let s' = file_read f in
  let () = file_close f in
  s = s'
))
