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
    } else if opt.c {
        println!("複数列にソートしようとしましたが、'ks'というコマンドにそんな機能はありません～")
    } else if opt.r {
        println!("ks.lock ks.toml LICENCE ks.md 'ks' \n\n ./ks:\nks.rs")
    } else if opt.one {
        println!("や\n～\nい\n！\nカ\nス\nカ\nス\n！");
    } else {
        println!("や～い！カスカス！");
    }

}
