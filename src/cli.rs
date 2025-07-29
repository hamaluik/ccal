//   Copyright 2025 Kenton Hamaluik
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use clap::{ColorChoice, Parser};
use jiff::civil::Date;

#[derive(Parser, Debug)]
#[command(author = clap::crate_authors!(), version, about, long_about = None, help_template = "\
{before-help}{name} {version}
by {author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, env, default_value_t = ColorChoice::Auto)]
    /// Control whether color is used in the output
    pub colour: ColorChoice,

    #[arg(short, long, env, value_parser = date_parser)]
    /// The start date in YYYY-MM-DD format
    pub start: Option<Date>,

    #[arg(short, long, env, default_value_t = 4)]
    /// The number of weeks to render
    pub nweeks: u8,

    #[arg(short, long, env)]
    /// The year to render; if specified overrides the start and nweeks settings
    pub year: Option<i16>,

    #[arg(long = "no-header", env, default_value_t = false)]
    /// Don't print the header
    pub no_header: bool,

    #[arg(long = "no-weeknums", env, default_value_t = false)]
    /// Don't print the week numbers
    pub no_weeknums: bool,

    #[arg(long = "no-months", env, default_value_t = false)]
    /// Don't print the month names
    pub no_months: bool,

    #[arg(long = "no-hl-today", env, default_value_t = false)]
    /// Don't highlight today's date
    pub no_hl_today: bool,
}

pub fn cli() -> Cli {
    Cli::parse()
}

fn date_parser(s: &str) -> Result<Date, &'static str> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err("Invalid date format. Use YYYY-MM-DD.");
    }
    let year: i16 = parts[0].parse().map_err(|_| "Invalid year")?;
    let month: i8 = parts[1].parse().map_err(|_| "Invalid month")?;
    let day: i8 = parts[2].parse().map_err(|_| "Invalid day")?;
    Ok(jiff::civil::date(year, month, day))
}
