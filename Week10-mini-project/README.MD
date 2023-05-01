# Rust exercise: Markdown Parser

## Introduction  
This program defines a function called parse_markdown that takes an input string and returns a parsed output string. The function iterates over each line in the input string and checks if it matches any of the Markdown syntax rules, such as headings (# ), lists (- ), and paragraphs. If a list is detected, the program sets a flag (in_list) to keep track of whether it's inside a list, and adds the appropriate HTML tags ul and li to the output string. If the program detects the end of a list, it closes the ul tag. Finally, the program returns the output string.

The main function reads a Markdown file called example.md into a string, passes it to the parse_markdown function, and prints the output string to the console.

This is a very basic Markdown parser that only handles a limited set of syntax rules. 


## Developer Guide
 * $`make format` to format code  
 * $`make lint` to lint  
 * $`cargo run` to run 
 
## Language
Rust

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
