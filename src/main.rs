extern crate structopt;

use rand::{thread_rng, Rng};
use std::iter;
use structopt::StructOpt;

const COMPANIES: usize = 10;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "g")]
    generate: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.generate {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let v: Vec<&str> = iter::repeat(())
                .map(|()| rng.gen())
                .take(COMPANIES + 1)
                .map(|x| match x {
                    false => "0",
                    true => "1",
                })
                .collect();
            println!("{}", v.join(" "));
        }
    }
}
