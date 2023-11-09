# Report CSV to HTML converter

Converter which reads CSV with report and converts it to HTML. Supports following content filters:
- `--date`
- `--author`

## Usage


```
cargo run -- input.csv --date "date-to-filter" >output.html
```

Example:

```
cargo run -- input.csv --date "9/10/2023" >out.html
```