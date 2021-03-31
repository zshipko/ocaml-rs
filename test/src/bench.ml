open Core_bench
open Ocamlrs_test

let () =
  let run_config = Bench.Run_config.create ~quota:(Bench.Quota.Num_calls 1000) () in
  Bench.bench ~run_config
    [ Bench.Test.create ~name:"func" Runtime.func
    ; Bench.Test.create ~name:"native_func" Runtime.native_func ]
