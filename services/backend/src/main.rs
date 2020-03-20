use structopt::StructOpt;
use term_size;
mod info;
mod openweather_service;
mod weatherbit_service;
mod geoip_service;

#[derive(StructOpt, Debug)]
#[structopt(name = "uvindex", about = "Fetch UV Index data from the terminal!")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbosity level (-v, -vv, -vvv, etc.)
    #[structopt(short, long = "verbose", parse(from_occurrences), global(true))]
    verbosity: u8,

    /// Subcommand
    #[structopt(subcommand)]
    cmd: Option<Subcommand>
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    /// Outputs UV index information chart
    #[structopt(name = "info")]
    Info,

    /// Outputs the current UV index
    #[structopt(name = "now")]
    Now,
}

fn on_info() {
    // Refactor when https://github.com/rust-lang/rust/issues/53667 is in a stable Rust release
    let (width, _) = term_size::dimensions_stdout().unwrap_or((0,0));
    if width >= 50 {
        info::print_info_table(width);
    } else {
        info::print_info_text();
    }
}

fn on_now(verbosity: u8) {
    let geoip_service::Position {latitude, longitude} = geoip_service::current_position().unwrap();

    let uv_index = match weatherbit_service::current_uv_index(latitude, longitude) {
        Ok(index) => index,
        Err(_) => openweather_service::current_uv_index(latitude, longitude).unwrap()
    };

    match verbosity {
        0 => println!("{}", uv_index),
        1 => println!("UV Index: {}. Risk: {}", uv_index, info::risk_factor(uv_index)),
        v if v >= 2 => println!("UV Index: {}. Risk: {}.\n{}", uv_index, info::risk_factor(uv_index), info::info_message(uv_index)),
        _ => panic!("Invalid verbosity provided.")
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        None => on_now(opt.verbosity),
        Some(Subcommand::Info) => on_info(),
        Some(Subcommand::Now) => on_now(opt.verbosity)
    }
}
