open Core_bench
open Ocamlrs_test

let () =
  Bench.bench
    [ Bench.Test.create ~name:"func" Runtime.func
    ; Bench.Test.create ~name:"native_func" Runtime.native_func ]
