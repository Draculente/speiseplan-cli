use dirs::config_dir;
use speiseplan_cli::cli::Cli;
use speiseplan_cli::cli::CliCommand;
use clap::Parser;
use speiseplan_cli::config::Config;
use speiseplan_cli::model::Allergen;
use speiseplan_cli::model::Data;
use speiseplan_cli::model::Location;
use speiseplan_cli::model::Meal;
use speiseplan_cli::view::Context;
use speiseplan_cli::view::View;
use anyhow::anyhow;


fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}

fn run() -> anyhow::Result<()> {
    let path = config_dir().ok_or(anyhow!("Failed to get the path of the config directory."))?;
    let path = format!("{}{}", path.to_str().ok_or(anyhow!("Failed to compile config path"))?,  "/speiseplan-cli/config.toml");
    let config = Config::read_from_file(path.as_str())?;

    let cli = Cli::parse();

    let context = Context::new(config.clone(), cli.date()?, cli.command().clone());

    print!("{}", fetch_output(cli, context)?);

    Ok(())
}

fn fetch_output(cli: Cli, context: Context) -> anyhow::Result<String> {
    let url = cli.get_full_url(&context);
    //dbg!(&url);
    let mut response = ureq::get(url).call()?;
    let body = response.body_mut();
    let data = match cli.command() {
        CliCommand::Meals(_) => body.read_json::<Data<Vec<Meal>>>()?.render(&context),
        CliCommand::Locations => body.read_json::<Data<Vec<Location>>>()?.render(&context),
        CliCommand::Allergens => body.read_json::<Data<Vec<Allergen>>>()?.render(&context),
    };

    Ok(data)
}
