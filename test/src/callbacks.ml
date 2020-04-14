external apply1: ('a -> 'a) -> 'a -> 'a = "apply1"
external apply3: ('a -> 'a) -> 'a -> 'a = "apply3"

let%test "apply1 float" = (apply1 (( +. ) 1.0) 2.5 = 3.5)
let%test "apply3 float" = (apply3 (( +. ) 1.0) (-1.0) = 2.0)
let%test "apply3 string" = (apply3 (( ^ )  "A") "A" = "AAAA")
let%test "apply3 apply1" = (apply3 (apply1 (( +. ) 1.0)) 1000.0 = 1003.0)

let%test "apply1 failure" =
  try apply1 (fun _ -> failwith "Testing") true
  with
    | Failure x -> x = "Testing"
    | _ -> false

let%test "apply3 invalid_arg" =
  try apply3 (fun _ -> invalid_arg "Testing") true
  with
    | Invalid_argument x -> x = "Testing"
    | _ -> false

external apply_range: (int list -> 'a) -> int -> int -> 'a = "apply_range"

let%test "apply range 1" =
  (apply_range (List.map (( + ) 1)) 0 10 = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10])
