mod day01;

static AOC_DRIVER_FUNCS: &'static [fn()] = &[day01::utilities::compute];

fn main() {
    for aoc_compute_helper in AOC_DRIVER_FUNCS {
        aoc_compute_helper();
    }
}