# How To Use The Template

Begin by copy-pasting the template directory into a new name (it should start with a letter). You may want to delete the existing year directory with my answers and use those names!

Add your personal input (the contents of `https://adventofcode.com/<YEAR>/day/<DAY>/input`) to the relevant input file (e.g. `year<YEAR>/inputs/<DAY>`).

Add your code to the relevant src file (e.g. `year<YEAR>/src/day<DAY>.rs`)

Recommend changing the top level function's input parameter to remove the leading underscore (it's there to stop Rust complaining when the functions are empty).
The input is provided as a `&[Vec<String>]`. Your input lines are split into top level slice elements split on double-line breaks, with individual lines forming the Strings of a lower-level slice.
For days where the input doesn't have any double-line splits, you may want the first line of your code to be let input_lines = input_lines[0] for simplicity.
The output required is a pair of Strings, which will be printed to terminal. In the vast majority of days, the result values are numbers, but occasionally strings are wanted!
The test framework in each file can also be used to put example cases from the puzzle page with the example answers inline, following the comments.
To run the code, simply run cargo run `<DAY>` from within your copied directory. Remember to use --release if you want to compare run-times!
