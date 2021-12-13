mod day01;
mod day02;

static AOC_DRIVER_FUNCS: &'static [fn()] = &[
    day01::runner::compute,
    day02::runner::compute
];

fn main() {
    for aoc_compute_helper in AOC_DRIVER_FUNCS {
        aoc_compute_helper();
    }
}