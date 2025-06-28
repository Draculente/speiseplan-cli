use crate::cli::CliCommand;
use crate::config::Config;
use crate::model::Allergen;
use crate::model::Data;
use crate::model::Language;
use crate::model::Location;
use crate::model::Meal;
use crate::model::MealPrice;
use chrono::DateTime;
use chrono::Local;
use console::Style;

pub struct Context {
    pub config: Config,
    command: CliCommand,
    pub date: DateTime<Local>,
}

impl Context {
    pub fn new(config: Config, date: DateTime<Local>, command: CliCommand) -> Self {
        Self {
            config,
            command,
            date,
        }
    }
}

pub trait View {
    fn render(&self, context: &Context) -> String;
}

impl View for Language {
    fn render(&self, _: &Context) -> String {
        format!("{} ({})", self.name, self.code)
    }
}

impl View for Allergen {
    fn render(&self, _: &Context) -> String {
        format!("{} ({})", self.name, self.code)
    }
}

impl View for Location {
    fn render(&self, context: &Context) -> String {
        let italic = Style::new().italic();
        if context.command == CliCommand::Locations {
            format!(
                "{} {} ({})\n{}",
                self.city,
                self.name,
                self.code,
                italic.apply_to(if let Some(ref e) = self.available_languages.as_ref() {
                    e.iter()
                        .map(|e| e.render(context))
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    "Error".to_owned()
                })
            )
        } else {
            format!("{} {}", self.name, self.city)
        }
    }
}

impl View for Meal {
    fn render(&self, context: &Context) -> String {
        let italic = Style::new().italic();
        let emoji = if self.vegan {
            "🌱"
        } else if self.vegetarian {
            "🥚"
        } else {
            "🥩"
        };
        format!(
            "{} {}\n{} - {}",
            emoji,
            self.name,
            self.price.render(context),
            italic.apply_to(&self.location.render(context))
        )
    }
}

impl View for MealPrice {
    fn render(&self, context: &Context) -> String {
        match context
            .config
            .price_category
            .as_ref()
            .map(|e| e.as_str())
            .unwrap_or("none")
        {
            "student" | "students" | "s" => format!("{:.2}€", self.students),
            "guest" | "guests" | "g" => format!("{:.2}€", self.guests),
            "employee" | "employees" | "e" => format!("{:.2}€", self.employees),
            _ => format!(
                "{:.2}€/{:.2}€/{:.2}€",
                self.students, self.employees, self.guests
            ),
        }
    }
}

impl<U: View> View for Data<Vec<U>> {
    fn render(&self, context: &Context) -> String {
        let converted: DateTime<Local> = DateTime::from(self.last_updated);
        format!(
            "Data for: {}\n\n\n{} \n\n\nLast updated: {}\n",
            context.date.format("%a, %d.%m.%Y"),
            self.data
                .iter()
                .map(|e| e.render(context))
                .collect::<Vec<String>>()
                .join("\n\n"),
            converted
        )
    }
}
