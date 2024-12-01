# advent of code 2024

## disclaimer
- this is my repo for the advent of code in 2024
- i usually use these riddles to learn a new language. this time its rust!
- AI usage will be disabled and will only be used to ask stuff about language specific questions, like syntax or general doc questions, rubber duck replacement so to say

## setup
### create skeleton files
i added a `sh` script to create skeleton files for each day, it also sets up folders on every execute:
- `./new_day.sh 1` creates an empty input file for day 1, a skeleton for the day1 solving rust file and adds the day1 file to the main module file

### main file
`src/main.rs` runs the given day and either prints the output to console or creates an output file.
cargo watch usage e.g.: 
- `cargo watch -x 'run -- 1 0'` runs day 1 with console output
- `cargo watch -x 'run -- 1 1'` runs day 1 with output to file

