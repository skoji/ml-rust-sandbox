extern crate structopt;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
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
        let value_range = Uniform::new_inclusive(0, 1);
        for _ in 0..100 {
            let v: Vec<&str> = value_range
                .sample_iter(&mut rng)
                .take(COMPANIES + 1)
                .map(|x| match x {
                    0 => "0",
                    1 => "1",
                    _ => "",
                })
                .collect();
            println!("{}", v.join(" "));
        }
    }
}
