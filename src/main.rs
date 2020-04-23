use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    distance: String,
    time: String,
}


fn main() {
    let options = Options::from_args();
}
