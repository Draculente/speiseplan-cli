use dirs::config_dir;
use crate::cli::Cli;
use crate::cli::Command;
use clap::Parser;
use speiseplan_cli::config::Config;
use speiseplan_cli::model::Allergen;
use speiseplan_cli::model::Data;
use speiseplan_cli::model::Location;
use speiseplan_cli::model::Meal;
use speiseplan_cli::view::Context;
use speiseplan_cli::view::View;
use anyhow::anyhow;

/*
 * TODO: Ich möchte ein anderes Datum als heute angeben können
 * TODO: Für die Allergene und die Location sollte keine voll ausgefüllte Config notwendig sein
 */

mod cli;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}

fn run() -> anyhow::Result<()> {
    let path = config_dir().ok_or(anyhow!("Failed to get the path of the config directory."))?;
    let path = format!("{}{}", path.to_str().ok_or(anyhow!("Failed to compile config path"))?,  "/mensa/config.toml");
    let config = Config::read_from_file(path.as_str())?;

    let cli = Cli::parse();

    let context = Context::new(config.clone(), cli.date()?);

    print!("{}", fetch_output(cli, context)?);

    Ok(())
}

fn fetch_output(cli: Cli, context: Context) -> anyhow::Result<String> {
    let url = cli.get_full_url(&context);
    //dbg!(&url);
    let response = reqwest::blocking::get(url)?;
    let data = match cli.command() {
        Command::Meals(_) => response.json::<Data<Vec<Meal>>>()?.render(&context),
        Command::Locations => response.json::<Data<Vec<Location>>>()?.render(&context),
        Command::Allergens => response.json::<Data<Vec<Allergen>>>()?.render(&context),
    };

    Ok(data)
}
