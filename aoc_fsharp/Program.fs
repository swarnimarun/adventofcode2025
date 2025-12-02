let parseRotation (src: string): int =
    match src[0] with
    | 'L' -> -1 * (src.Substring 1 |> int)
    | _ -> src.Substring(1) |> int

module Day1 =
    module Part1 =
        let run (input: string) =
            let mutable acc = 50
            let res =
                input.Trim().Split [|'\n'|]
                |> Array.map parseRotation
                |> Array.filter (fun (x) ->
                    acc <- (((acc + x) % 100) + 100) % 100
                    acc = 0
                )
                |> Array.length
            printfn "Result for the part is %d" res
    module Part2 =
        let run (input: string) = ()

exception StringException of string

let readInputToString = System.IO.File.ReadAllText

[<EntryPoint>]
let main args =
    let arg_day = args[0].Substring(3) |> int
    let day =
        match arg_day with
        | 1 -> (Day1.Part1.run, Day1.Part2.run)
        | _ ->
            raise <|
            StringException "Invalid day selected: only supported format [day1, Day1, day2,..etc]"

    let arg_part = args[1].Substring(4) |> int
    let f = match arg_part with
                  | 1 -> fst day
                  | 2 -> snd day
                  | _ -> StringException "Invalid part selected: only supported [part1, part2]" |> raise
    printfn "Running for Day %d Part %d" arg_day arg_part
    args[2] |> readInputToString |> f

    0
