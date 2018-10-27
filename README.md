# conway-game-of-life-rs
A rust implementation of ["Conway's Game of life"](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

## CLI Examples

Some example patterns taken from wikipedia.

#### Two Blinkers
`$ cargo run -- -pattern1 "3,3;3,4;3,5" -pattern2 "10,5;11,5;12,5"`

[![asciicast](https://asciinema.org/a/zd3LGVvKuuyvyXPOA75iRerqf.png)](https://asciinema.org/a/zd3LGVvKuuyvyXPOA75iRerqf)

#### Toad

`$ cargo run -- -pattern "5,7;5,6;6,5;7,8;8,7;8,6"`
[![asciicast](https://asciinema.org/a/b0yW2ICscA5wIrfeZFlQSCqwE.png)](https://asciinema.org/a/b0yW2ICscA5wIrfeZFlQSCqwE)


#### Glider

`$ cargo run -- -pattern "3,10;3,9;3,8;2,8;1,9"`

[![asciicast](https://asciinema.org/a/5DL1cWhrkI9ZhdJj3nSNk3hD4.png)](https://asciinema.org/a/5DL1cWhrkI9ZhdJj3nSNk3hD4)

# Building

Run 

`$ cargo build`

# Running in CLI

Run 

`$ cargo run`