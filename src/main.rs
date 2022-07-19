use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "l")]
    ls: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.ls {
        println!("や～い！'ls'と間違えるお前に権限はなーい！");
    } else {
        println!("や～い！カスカス！");
    }
}
