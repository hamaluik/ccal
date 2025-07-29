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

use ansi_term::Style;
use color_eyre::Result;
use jiff::{ToSpan, civil::Date};
use std::io::IsTerminal;

mod cli;

fn today() -> jiff::civil::Date {
    use jiff::Zoned;

    let zdt = Zoned::now();
    zdt.date()
}

fn first_day_of_week(day: &Date) -> Date {
    let weekday = day.weekday();
    let weekday = weekday.to_sunday_zero_offset();
    day.saturating_sub(weekday.days())
}

fn next_week(sunday: &jiff::civil::Date) -> Date {
    sunday.saturating_add(7.days())
}

fn week_contains_new_month(sunday: &Date) -> bool {
    let next_sunday = next_week(sunday);
    sunday.month() != next_sunday.month()
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let config = cli::cli();
    let use_colours = match config.colour {
        clap::ColorChoice::Never => false,
        clap::ColorChoice::Always => true,
        _ => std::io::stdout().is_terminal(),
    };

    let header_style = if use_colours {
        Style::new().bold().fg(ansi_term::Color::Purple)
    } else {
        Style::new()
    };

    let today_style = if use_colours {
        Style::new()
            .bold()
            .fg(ansi_term::Color::Black)
            .on(ansi_term::Color::White)
    } else {
        Style::new()
    };

    let week_num_style = if use_colours {
        Style::new().bold().fg(ansi_term::Color::Yellow)
    } else {
        Style::new()
    };

    let month_style = if use_colours {
        Style::new().bold().fg(ansi_term::Color::Green)
    } else {
        Style::new()
    };

    if !config.no_header {
        if !config.no_weeknums {
            print!("{}", header_style.paint("Wk "));
        }
        if !config.no_months {
            print!("    ");
        }
        println!("{}", header_style.paint("Su Mo Tu We Th Fr Sa"));
        print!("--------------------");
        if !config.no_weeknums {
            print!("---");
        }
        if !config.no_months {
            print!("----");
        }
        println!();
    }

    // parse / figure out the start date
    let mut start = if let Some(year) = &config.year {
        first_day_of_week(&jiff::civil::date(*year, 1, 1))
    } else if let Some(start_date) = &config.start {
        first_day_of_week(start_date)
    } else {
        first_day_of_week(&today())
    };
    let num_weeks = if config.year.is_some() {
        53 // round up from 52
    } else {
        config.nweeks
    };
    for _ in 0..num_weeks {
        if !config.no_weeknums {
            let week_num = start.day_of_year() / 7 + 1;
            let week_num = (week_num + 1) % 53;
            let week_num = if week_num == 0 { 53 } else { week_num };
            print!("{} ", week_num_style.paint(format!("{week_num:>2}")));
        }
        if !config.no_months {
            let month = if week_contains_new_month(&start) {
                let month = (start.month() + 1) % 12;
                match month {
                    0 => "Dec",
                    1 => "Jan",
                    2 => "Feb",
                    3 => "Mar",
                    4 => "Apr",
                    5 => "May",
                    6 => "Jun",
                    7 => "Jul",
                    8 => "Aug",
                    9 => "Sep",
                    10 => "Oct",
                    11 => "Nov",
                    12 => "Dec",
                    13 => "Jan",
                    _ => unreachable!(),
                }
            } else {
                "   "
            };
            print!("{} ", month_style.paint(format!("{month:>3}")));
        }

        // print the week
        let mut week: Vec<String> = Vec::new();
        let sunday = start;
        for i in 0..7 {
            let day = sunday.saturating_add(i.days());

            if !config.no_hl_today && day == today() {
                week.push(
                    today_style
                        .paint(format!("{day:>2}", day = day.day()))
                        .to_string(),
                );
            } else {
                week.push(format!("{day:>2}", day = day.day()));
            }
        }
        println!("{}", week.join(" "));

        start = next_week(&start);
    }

    Ok(())
}
