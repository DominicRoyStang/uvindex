use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "uvindex")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

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
    println!("info: TODO");
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
    println!("{:#?}", opt);
}
