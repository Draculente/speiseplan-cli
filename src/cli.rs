use crate::url::UrlParams;
use crate::view::Context;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Days;
use chrono::Local;
use chrono::Timelike;
use chrono::Weekday;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct MealArgs {
    #[arg(short)]
    day: Option<String>,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub enum CliCommand {
    Meals(MealArgs),
    Locations,
    Allergens,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

impl Cli {
    pub fn command(&self) -> CliCommand {
        self.command
            .clone()
            .unwrap_or(CliCommand::Meals(MealArgs { day: None }))
    }

    pub fn get_full_url(&self, context: &Context) -> String {
        let url_params = UrlParams::new();
        let appendix = match self.command() {
            CliCommand::Meals(_) => format!(
                "/meals{}",
                url_params
                    .add_monad("vegan", context.config.vegan)
                    .add_monad("vegetarian", context.config.vegetarian)
                    .add_monad("language", context.config.language.as_ref())
                    .add("location", context.config.location_codes.join(","))
                    .add(
                        "exclude_allergens",
                        context.config.exclude_allergens.join(",")
                    )
                    .add("date", context.date.format("%+"))
                    .build()
            ),
            CliCommand::Locations => format!("/locations"),
            CliCommand::Allergens => {
                format!(
                    "/allergens{}",
                    UrlParams::new()
                        .add("location", context.config.location_codes.join(","))
                        .add_monad("language", context.config.language.as_ref())
                        .build()
                )
            }
        };

        format!("{}{}", context.config.url, appendix)
    }

    pub fn date(&self) -> anyhow::Result<DateTime<Local>> {
        let now = chrono::Local::now();
        if let CliCommand::Meals(meal_args) = self.command() {
            if let Some(day) = meal_args.day {
                let day = workdays::parse_weekday(day.as_str())
                    .ok_or(anyhow!("Failed to parse weekday."))?;
                return now
                    .checked_add_days(Days::new(day.days_since(now.weekday()).into()))
                    .ok_or(anyhow!("Failed to calcuate date to show"));
            }
        }
        Ok(get_next_open_mensa_date(chrono::Local::now()))
    }
}

fn get_next_open_mensa_date(now: DateTime<Local>) -> DateTime<Local> {
    let show_day = if now.hour() >= 15 || now.weekday().is_weekend() {
        if now.weekday().is_weekend() || now.weekday() == Weekday::Fri {
            now.checked_add_days(Days::new(Weekday::Mon.days_since(now.weekday()).into()))
                .expect("There is a next monday.")
        } else {
            now.checked_add_days(Days::new(1))
                .expect("There is a next day.")
        }
    } else {
        now
    };
    show_day
}

trait Weekend {
    fn is_weekend(&self) -> bool;
}

impl Weekend for Weekday {
    fn is_weekend(&self) -> bool {
        match self {
            Weekday::Mon => false,
            Weekday::Tue => false,
            Weekday::Wed => false,
            Weekday::Thu => false,
            Weekday::Fri => false,
            Weekday::Sat => true,
            Weekday::Sun => true,
        }
    }
}
