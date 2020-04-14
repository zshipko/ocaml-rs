external list_length: 'a list -> int = "list_length"

let%test "list length (empty)" = (list_length [] = 0)
let%test "list length (small)"= (list_length [1; 2; 3] = 3)
let%test "list length (big)" = (list_length (Array.make 10000 0 |> Array.to_list) = 10000)
