use structopt::StructOpt;
use uom::si::f32::{Length, Time};

#[derive(StructOpt, Debug)]
struct Options {
    distance: String,
    time: String,
}

struct Run {
    distance: Length,
    time: Time,
}

impl Run {
    fn from_options(options: &Options) -> Self {
        
    }
}

fn main() {
    let options = Options::from_args();
    dbg!(options);
}
