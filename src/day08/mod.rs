
pub mod input;
pub mod distance;
pub mod junction_box;
pub mod circuit;
pub mod part1;
pub mod part2;

pub fn run() -> (Option<usize>, Option<usize>) {
    let input = input::read(input::INPUT);
    (
        part1::solve(&input, 1000, true),
        part2::solve(&input)
    )
}
