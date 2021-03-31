open Bechamel
open Toolkit
open Ocamlrs_test

let test_func =
  Test.make ~name:"func"
    (Staged.stage Runtime.func)

let test_native_func =
  Test.make ~name:"native_func"
    (Staged.stage Runtime.func)


let tests = (Test.make_grouped ~name:"call overhead" ~fmt:"%s %s" [ test_func; test_native_func ])

let cfg = Benchmark.cfg ~limit:2000 ~quota:(Time.second 5.0) ~kde:(Some 1000) ()

let benchmark () =
  let ols =
    Analyze.ols ~bootstrap:0 ~r_square:true ~predictors:Measure.[| run |] in
  let instances = Instance.[ monotonic_clock ] in
  let raw_results = Benchmark.all cfg instances tests in
  let results =
    List.map (fun instance -> Analyze.all ols instance raw_results) instances
  in
  let results = Analyze.merge ols instances results in
  (results, raw_results)

let () =
  List.iter
    (fun v -> Bechamel_notty.Unit.add v (Measure.unit v))
    Instance.[ minor_allocated; major_allocated; monotonic_clock ]

let img (window, results) =
  Bechamel_notty.Multiple.image_of_ols_results ~rect:window
    ~predictor:Measure.run results

open Notty_unix

let () =
  let window =
    match winsize Unix.stdout with
    | Some (w, h) -> { Bechamel_notty.w; h }
    | None -> { Bechamel_notty.w = 80; h = 1 } in
  let results, _ = benchmark () in
  print_endline "\nBenchmarks";
  img (window, results) |> eol |> output_image
