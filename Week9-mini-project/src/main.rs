use std::fs;

fn parse_markdown(input: &str) -> String {
    let mut output = String::new();
    let mut in_list = false;

    for line in input.lines() {
        if line.starts_with("# ") {
            output.push_str(&format!("<h1>{}</h1>", &line[2..]));
        } else if line.starts_with("## ") {
            output.push_str(&format!("<h2>{}</h2>", &line[3..]));
        } else if line.starts_with("### ") {
            output.push_str(&format!("<h3>{}</h3>", &line[4..]));
        } else if line.starts_with("- ") {
            if !in_list {
                output.push_str("<ul>");
                in_list = true;
            }
            output.push_str(&format!("<li>{}</li>", &line[2..]));
        } else {
            if in_list {
                output.push_str("</ul>");
                in_list = false;
            }
            output.push_str(&format!("<p>{}</p>", line));
        }
    }

    if in_list {
        output.push_str("</ul>");
    }

    output
}

fn main() {
    let input = fs::read_to_string("example.md").unwrap();
    let output = parse_markdown(&input);
    println!("{}", output);
}
