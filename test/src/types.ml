external list_length: 'a list -> int = "list_length"

let%test "list length (empty)" = (list_length [] = 0)
let%test "list length (small)"= (list_length [1; 2; 3] = 3)
let%test "list length (big)" = (list_length (Array.make 10000 0 |> Array.to_list) = 10000)

external list_nil: unit -> 'a list = "list_nil"
external list_cons: 'a list -> 'a -> 'a list = "list_cons"

let%test "list nil" = (list_nil () = [])
let%test "list cons 1" = (list_cons (list_nil ()) 12.5 = [12.5])
let%test "list cons 2" = (list_cons (list_cons (list_nil ()) 12.5) 11.5 = [11.5; 12.5])
