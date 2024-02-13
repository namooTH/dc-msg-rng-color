use cli_clipboard;
use std::collections::HashMap;
use rand::Rng;

fn main() {
    println!("\nenter text:\n");
    let mut input = String::new();
    let _userinput = std::io::stdin().read_line(&mut input).unwrap();

    let mut text: String = "```ansi\n".to_string();
    let mut colours: HashMap<String, String>= HashMap::new();
    // colours.insert("black".to_string(), "0;30m".to_string());
    colours.insert("red".to_string(), "0;31m".to_string());
    colours.insert("green".to_string(), "0;32m".to_string());
    colours.insert("yellow".to_string(), "0;33m".to_string());
    colours.insert("blue".to_string(), "0;34m".to_string());
    colours.insert("purple".to_string(), "0;35m".to_string());
    colours.insert("cyan".to_string(), "0;36m".to_string());
    // colours.insert("white".to_string(), "0;37m".to_string());

    let endchar: String = "[0m".to_string();
    let startchar: String = "[".to_string();

    for i in input.chars() {
        let mut rng = rand::thread_rng();
        let randomcolor: String = format!("{}{}",startchar, Vec::from_iter(colours.values())[rng.gen_range(0..colours.len())]);
        text = format!("{}{}{}{}", text, randomcolor, i, endchar)
    }

    text.truncate(text.len() - 4);
    text = format!("{}{}", text, "```");
    cli_clipboard::set_contents(text.to_owned()).unwrap();
    println!("\ntext copied to clipboard!!11!1");
    println!("press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
