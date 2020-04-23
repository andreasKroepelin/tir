# Today I Ran

*Today I Ran* (tir) is a minimalist tool to analyse one's running performance.
If you stay away from fancy fitness watches or tracking apps for your smartphone and all you do is
* measure your time
* measure the distance (the respective feature on Google Maps works quite well)
when you go for a run, then *Today I Ran* might be what you need.

Currently, *Today I Ran* has three features:
* calculate your average velocity
* estimate the time you would need for certain other distances (given your average velocity)
* compare your average velocity with that of other performances

## Usage

The command line tool is very simple to use:
```
$ tir 14.3km 1h12min4s
Today, you ran 14.3 km in 1 h 12 min 4 s.
Your average velocity was 11.906 km/h.
```

To get an extended output, use the `-v` or `--verbose` flag:
```
$ tir -v 14.3km 1h12min4s
Today, you ran 14.3 km in 1 h 12 min 4 s.
Your average velocity was 11.906 km/h.

This is how long you would have needed for other distances:
         100 m  30 s
          1 km  5 min 2 s
          5 km  25 min 11 s
         10 km  50 min 23 s
 half marathon  1 h 46 min 19 s
      marathon  3 h 32 min 38 s

Your average velocity compared to those of other performances:
 2.313 times  Ashprihanal Aalto's 3100 mi (longest ultra marathon) WR
 0.844 times  Yohann Diniz' 50 km race walk WR
 0.563 times  Eliud Kipchoge's inofficial marathon WR
 0.522 times  Kenenisa Bekele's 10000 m WR
 0.317 times  Usain Bolt's 100 m WR
 0.197 times  Cheetah Sarah's 100 m animal WR
```

The time parameter must be of the form `<x> h <y> min <z> s` where the spaces can be omitted (use `" "` in the terminal if you want to use spaces here!) and each part (hours, minutes and seconds) can be omitted as well.

The distance parameter accepts a quantity with on of the following units: meter, kilometer, yard, foot, mile.
All output uses kilometer (and kilometer per hour) per default.
To switch to using miles, use the flag `-m` or `--miles`.
Again, you are allowed to write `13 km` instead of `13km` but remember using `" "` then.

## Installation from source
Note that this tool is written in [Rust](https://www.rust-lang.org/) so you need to have a Rust development environment installed.
Clone this repository and run `cargo build --release` inside the `tir` folder.
