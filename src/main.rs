// Copyright (c) 2017-2019, scoobybejesus
// Redistributions must include the license: https://github.com/scoobybejesus/cryptools/blob/master/LEGAL.txt

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
// Note: the above are possibly temporary, to silence "x was not used" warnings.
// #[warn(dead_code)] is the default (same for unused_variables)


use std::ffi::OsString;
use std::path::PathBuf;
use std::error::Error;
use std::io;
use std::time::Duration;

use ::tui::Terminal;
use ::tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use termion::event::Key;
use structopt::StructOpt;

mod account;
mod transaction;
mod core_functions;
mod csv_import_accts_txns;
mod create_lots_mvmts;
mod import_cost_proceeds_etc;
mod cli_user_choices;
mod csv_export;
mod txt_export;
mod string_utils;
mod decimal_utils;
mod tests;
mod wizard;
mod skip_wizard;
mod setup;
mod tui;
mod export_all;



#[derive(StructOpt, Debug)]
#[structopt(name = "cryptools")]
pub(crate) struct Cli {

    #[structopt(flatten)]
    flags: Flags,

    #[structopt(flatten)]
    opts: Options,

    /// File to be imported.  (Currently, the only supported date format is %m/%d/%y.)
    #[structopt(name = "file", parse(from_os_str))]
    file_to_import: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub(crate) struct Flags {

    /// User is instructing the program to skip the data entry wizard.
    /// When set, program will error without required command-line args.
    #[structopt(name = "accept args", short = "a", long = "accept")]
    accept_args: bool,

    /// This will cause the program to expect the txDate field in the file_to_import to use the format
    /// YYYY-MM-dd or YY-MM-dd (or YYYY/MM/dd or YY/MM/dd, depending on the date-separator option)
    /// instead of the default US-style MM-dd-YYYY or MM-dd-YY (or MM/dd/YYYY or MM/dd/YY, depending on the
    /// date separator option).
    #[structopt(name = "date conforms to ISO 8601", short = "i", long = "iso")]
    iso_date: bool,

    /// Once the import file has been fully processed, the user will be presented
    /// with a menu for manually selecting which reports to print/export.
    #[structopt(name = "print menu", short, long = "print-menu")]
    print_menu: bool,

    /// This will prevent the program from writing reports to files.
    /// This will be ignored if -a is not set (the wizard will always ask to output).
    #[structopt(name = "suppress reports", short, long = "suppress")]
    suppress_reports: bool,
}

#[derive(StructOpt, Debug)]
pub(crate) struct Options {

    /// Choose "h", "s", or "p" for hyphen, slash, or period (i.e., "-", "/", or ".") to indicate the separator
    /// character used in the file_to_import txDate column (i.e. 2017/12/31, 2017-12-31, or 2017.12.31).
    #[structopt(name = "date separator character", short, long = "date-separator", default_value = "h", parse(from_os_str))]
    date_separator: OsString,

    /// Home currency (currency in which all resulting reports are denominated).
    /// (Only available as a command line setting.)
    #[structopt(name = "home currency", short = "c", long = "currency", default_value = "USD", parse(from_os_str))]
    home_currency: OsString,

    /// Cutoff date through which like-kind exchange treatment should be applied.
    /// Please use %y-%m-%d (or %Y-%m-%d) format for like-kind cutoff date entry.
    #[structopt(name = "like-kind cutoff date", short, long = "lk-cutoff", parse(from_os_str))]
    lk_cutoff_date: Option<OsString>,

    /// Inventory costing method (in terms of lot selection, i.e., LIFO, FIFO, etc.).
    /// There are currently four options (1 through 4).
    #[structopt(name = "method number for lot selection", short, long, default_value = "1", parse(from_os_str), long_help =
    r"    1. LIFO according to the order the lot was created.
    2. LIFO according to the basis date of the lot.
    3. FIFO according to the order the lot was created.
    4. FIFO according to the basis date of the lot.
    ")]
    inv_costing_method: OsString,

    /// Output directory for exported reports.
    #[structopt(name = "output directory", short, long = "output", default_value = ".", parse(from_os_str))]
    output_dir_path: PathBuf,
}



fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::from_args();

    println!(
    "
    Hello, crypto-folk!  Welcome to cryptools!

    This software will import your csv file's ledger of cryptocurrency transactions.
    It will then process it by creating 'lots' and posting 'movements' to those lots.
    Along the way, it will keep track of income, expenses, gains, and losses.

    Note: it is designed to import a full history. Gains and losses may be incorrect otherwise.
    ");

    let (input_file_path, settings) = setup::run_setup(args)?;

    let (
        account_map,
        raw_acct_map,
        action_records_map,
        transactions_map,
    ) = core_functions::import_and_process_final(input_file_path, &settings)?;

    let mut should_export_all = settings.should_export;
    let present_print_menu_tui = settings.print_menu;

    if present_print_menu_tui { should_export_all = false }

    if should_export_all {

        export_all::export(
            &settings,
            &action_records_map,
            &raw_acct_map,
            &account_map,
            &transactions_map
        )?;
    }

    if present_print_menu_tui {

        use crate::tui::event::{Events, Event, Config};

        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let mut app = tui::app::PrintWindow::new("Reports");

        let events = Events::with_config(Config {
            tick_rate: Duration::from_millis(250u64),
            ..Config::default()
        });

        loop {

            tui::ui::draw(&mut terminal, &app)?;

            match events.next()? {

                Event::Input(key) => match key {

                    Key::Char(c) => {
                        app.on_key(c);
                    }
                    Key::Up => {
                        app.on_up();
                    }
                    Key::Down => {
                        app.on_down();
                    }
                    _ => {}
                },
                _ => {}
            }

            if app.should_quit {
                break;
            }
        }

        // Seem to need both of these for the native terminal to be available for println!()'s below
        std::mem::drop(terminal);
        std::thread::sleep(Duration::from_millis(10));

        tui::app::export(
            &app,
            &settings,
            &action_records_map,
            &raw_acct_map,
            &account_map,
            &transactions_map
        )?;

    }

    // use tests::test;
    // test::run_tests(
    //     &transactions_map,
    //     &action_records_map,
    //     &account_map
    // );


    Ok(())

}
