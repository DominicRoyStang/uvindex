use structopt::StructOpt;
use term_size;
mod info;

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

    /// Outputs an 8-day UV index forecast
    #[structopt(name = "forecast")]
    Forecast
}

fn on_info() {
    if let Some((width, _)) = term_size::dimensions_stdout() {
        if width >= 50 {
            info::print_info_table(width);
        } else {
            info::print_info_text();
        }
    } else {
        info::print_info_text();
    }
}

fn on_now() {
    println!("now: TODO");
}

fn on_forecast() {
    println!("forecast: TODO");
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        None => on_now(),
        Some(Subcommand::Info) => on_info(),
        Some(Subcommand::Now) => on_now(),
        Some(Subcommand::Forecast) => on_forecast()
    }
}
