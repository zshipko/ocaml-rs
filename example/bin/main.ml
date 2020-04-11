open Example

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

    for i = 1 to 10 do
      (* new_tuple *)
      let (a, b, c) = new_tuple i in
      Printf.printf "%d %d %d\n" a b c;
      assert (a = i && b = 2 * i && c = 3 * i);
    done;


    for i = 1 to 10 do
      (* new_array *)
      let arr = new_array i in
      Array.iter (Printf.printf "%d\n") arr;
      assert (arr = [| 0; i; 2 * i; 3 * i; 4 * i |]);
    done;


    for i = 1 to 10 do
      (* new list *)
      let lst = new_list i in
      List.iter (Printf.printf "%d\n") lst;
      assert (lst = [0; i; 2 * i; 3 * i; 4 * i]);
    done;


    (* testing_callback *)
    testing_callback 5 10;

    (* raise Not_found *)
    try raise_not_found ()
    with Not_found -> print_endline "Got Not_found";

    try raise_failure ()
    with Failure f -> begin
      assert (f = "testing");
      print_endline ("Got failure: " ^ f);
    end;

    try raise_exc 123
    with Exc i -> assert (i = 123);

    (* send float *)
    let f = send_float 2.5 in
    Printf.printf "send_float: %f\n" f;
    flush stdout;
    assert (f = 5.0);

    (* send first variant *)
    print_endline "send_first_variant";
    assert (send_first_variant () = First (2.0));

    (* finalizer *)
    print_endline "final_value";
    let _ = final_value () in

    (* bigarray *)
    print_endline "bigarray create";
    let ba = array1 100000 in

    print_endline "bigarray iter";
    for i = 0 to Bigarray.Array1.dim ba - 1 do
      assert (ba.{i} = i mod 256)
    done;

    print_endline "bigarray Array1::of_slice";
    let s = "testing" in
    let ba = array2 s in
    for i = 0 to Bigarray.Array1.dim ba - 1 do
      assert (ba.{i} = String.get s i)
    done;

    print_endline "string test";
    assert (string_test "wow" = "testing");

    let l = make_list 250000 in
    let next = ref 0 in
    Printf.printf "make_list: %d\n" (List.length l);
    assert (List.length l = 250000);
    assert (List.for_all (fun i ->
      let r = i = !next in
      next := i + 1;
      r) l);

    for _ = 1 to 3 do
      let l = make_array 100000 in
      Printf.printf "make_array: %d\n" (Array.length l);
      assert (Array.length l = 100000);
      Gc.full_major ();
      Gc.minor()
    done;

    assert (call int_of_string "123" = 123);

    (* Records *)
    let r = {
      foo = "testing";
      bar = 123.;
    } in

    let r = format_my_record r in
    assert (r = "MyRecord { foo: \"testing\", bar: 123.0 }");
    print_endline r;

    (* Unboxed float *)
    assert (unboxed_float 1.0 3.0 = 2.0);

    assert (more_than_five_params 1. 1. 1. 1. 1. 1. = 6.0);

    let () = match hash_variant () with
    | `Abc x -> assert (x = 123)
    | _ -> assert false in

    let a = custom_value 123 in
    let b = custom_value 456 in
    assert (a = b);
    assert (custom_value_int a = 123);
    assert (custom_value_int b = 456);

    (* list_hd_len *)
    let (a, b) = list_hd_len [1; 2; 3] in
    assert (a = Some 1);
    assert (b = 3);

    let (a, b) = list_hd_len [] in
    assert (a = None);
    assert (b = 0);

    print_endline "cleanup";
    Gc.full_major ();
    Gc.minor()
