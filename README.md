# fmnl
Formats newlines of input with a given string.

## Usage
fmnl [formatter]

## Examples
Given input file example.txt
```
first
second
third
```
and calling it with ", " as the formatter
```bash
cat example.txt | target/release/fmnl ", "
```
will output
```
first, second, third
```
