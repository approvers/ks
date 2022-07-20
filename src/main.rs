use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    // ls -ls: lsでは権限等も含めて詳細表示する
    #[structopt(short = "l")]
    ls: bool,

    // ls -C: 垂直方向に複数列にソートして表示する
    #[structopt(short = "C")]
    c: bool,

    // ls -R: サブディレクトリを再帰的に表示する
    #[structopt(short = "R")]
    r: bool,

    // ls -1: 表示を一列にする
    #[structopt(short = "1")]
    one: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.ls {
        println!("や～い！'ls'と間違えるお前に権限はなーい！");
    } else {
        println!("や～い！カスカス！");
    }
}
