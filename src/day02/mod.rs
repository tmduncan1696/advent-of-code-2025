
pub mod input;
pub mod part1;
pub mod part2;

pub fn run() -> (Option<usize>, Option<usize>) {
    let input = input::read(input::INPUT);
    (
        part1::solve(&input),
        part2::solve(&input)
    )
}
