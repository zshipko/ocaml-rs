type enum1 = Empty | First of int | Second of string array

type struct1 = {a: int; b: float; mutable c: string option; d: string array option;}

external raise_exc: float -> unit = "raise_exc"

