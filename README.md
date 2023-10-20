# Ajour Catalog

This repository generates a JSON file which is used in [Ajour](https://github.com/ajour/ajour/).
The JSON file contains all available addons from:

- Curse (temporary disabled)
- Tukui
- WowInterface
- Hub

The JSON file is automatically updated every 6 hour.

## Usage

To generate a `catalog.json` file run:

```rust
cargo run -- catalog
```

## License

Ajour Catalog is released under the [GPL-3.0 License.](https://github.com/ajour/catalog/blob/main/LICENSE)
