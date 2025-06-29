# Speiseplan CLI

<p align="center">
  <img width="600" src="./demo.svg">
</p>

This command-line tool allows you to fetch the current meals from all canteens in Schleswig-Holstein. You can filter the meals based on allergens, whether they are vegetarian or not, and the location where they are offered (e.g. Mensa Lübeck or Cafeteria II Kiel).

## Installation

To use the Mensa CLI, you can download the binary for your operating system from the [Github Release](https://github.com/Draculente/speiseplan-cli/releases) page.


## Configuration

You can set your preferences in the config file, for example:

```toml
url = "https://speiseplan.mcloud.digital/v2"

exclude_allergens = ["GlW"]

# vegan = true

vegetarian = true

location_codes = ["HL_CA", "HL_BB", "HL_ME"]

language = "de"

price_category = "student"
```


## Similar project

This project uses the [Mensa API](https://github.com/Draculente/mensa-api).

- [Tray Application](https://github.com/Importantus/speiseplan-tray/) (Windows, Linux, MacOS)
- [KDE Plasma Widget](https://github.com/lomenzel/mensa) (Linux with KDE Plasma)
- [Android Widget](https://github.com/hoppjan/LuebeckMensaWidget) (Android)


## License

This project uses the MIT License. See [LICENSE](./LICENSE) for details.
