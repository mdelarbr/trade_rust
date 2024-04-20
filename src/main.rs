use clap::{Arg, Command};
mod data_manager;
use data_manager::SystemCommandExecutor;

fn main() {
    let matches = Command::new("Rust Trade Application")
        .version("0.1")
        .author("mdelarbr - mateo.delarbre@gmail.com")
        .about("Performs operations on financial data")
        .subcommand(
            Command::new("backtest")
                .about("Performs a backtest on financial data")
                .arg(Arg::new("symbol")
                    .help("The symbol to perform the backtest on, e.g., eurusd")
                    .required(true)
                    .index(1))
                .arg(Arg::new("start_date")
                    .help("The start date for the backtest (ex: YYYY-MM-DD)")
                    .required(true)
                    .index(2))
                .arg(Arg::new("end_date")
                    .help("The end date for the backtest (ex: YYYY-MM-DD)")
                    .required(true)
                    .index(3))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("backtest", sub_m)) => {
            let symbol = sub_m.get_one::<String>("symbol").unwrap();
            let start_date = sub_m.get_one::<String>("start_date").unwrap();
            let end_date = sub_m.get_one::<String>("end_date").unwrap();
            println!("Backtesting {} from {} to {}", symbol, start_date, end_date);

            let executor = SystemCommandExecutor;
            if let Err(e) =data_manager::get_data_from_dukas(&executor, symbol, start_date, end_date) {
                eprint!("Error : {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            println!("Invalid command or parameters. Use --help for more information.");
        }
    }
}
