use rand::RngExt;

include!(concat!(env!("OUT_DIR"), "/images.rs"));

fn main() {
    let mut rng = rand::rng();
    let idx = std::env::args()
        .nth(1)
        .and_then(|a| a.parse::<usize>().ok())
        .map(|n| n % IMAGES.len())
        .unwrap_or_else(|| rng.random_range(0..IMAGES.len()));
    let image = IMAGES[idx];

    match rng.random_range(0..12u8) {
        0 => colour::red_ln!("{}", image),
        1 => colour::green_ln!("{}", image),
        2 => colour::yellow_ln!("{}", image),
        3 => colour::blue_ln!("{}", image),
        4 => colour::magenta_ln!("{}", image),
        5 => colour::cyan_ln!("{}", image),
        6 => colour::white_ln!("{}", image),
        7 => colour::dark_red_ln!("{}", image),
        8 => colour::dark_green_ln!("{}", image),
        9 => colour::dark_yellow_ln!("{}", image),
        10 => colour::dark_blue_ln!("{}", image),
        _ => colour::dark_magenta_ln!("{}", image),
    }
}
