# splitters
Split files linewise

# Install
`cargo install`

# Use
`splitters <num-split> <file> <optional-out-name>`

`splitters 10 lots_of_data.csv` will produce lots_of_data-0.csv through lots_of_data-9.csv.


I attempted to make this efficient, but I am a Rust novice, so any tips on how to make it go faster are appreciated.  On my machine, it splits at about 100 mb/sec.
