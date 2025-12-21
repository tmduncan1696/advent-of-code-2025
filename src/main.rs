use aoc::{args::Args, run};

fn main() {

    let args = Args::build();

    let out = run(args);

    for x in out {
        x.print();
    };
}
