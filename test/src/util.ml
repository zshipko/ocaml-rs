let gc () =
  Gc.compact ();
  Gc.minor ();
  Gc.full_major ()

let check_leaks f =
  let () = gc () in
  let stat = (Gc.stat ()).live_blocks in
  let r = f () in
  let () = gc () in
  let stat1 = (Gc.stat ()).live_blocks in
  if stat1 > stat then
    Printf.printf "Potential GC leak detected: %d, %d\n" stat stat1;
    assert (stat >= stat1);
  r
