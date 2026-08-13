use colored::Colorize;
use roxmltree::{Document, Node};
use std::env;
use std::fs;

fn print_tree(node: Node, prefix: &str, is_last: bool) {
    let (marker, child_prefix) = if node.is_root() {
        ("", "")
    } else if is_last {
        ("└── ", "    ")
    } else {
        ("├── ", "│   ")
    };

    let mut display_str = String::new();

    if node.is_element() {
        let mut s = format!("{}", node.tag_name().name().bold().blue());

        // Attribute anhängen
        let attrs: Vec<String> = node
            .attributes()
            .map(|a| format!("{}={}", a.name().cyan(), format!("'{}'", a.value()).green()))
            .collect();
        if !attrs.is_empty() {
            s.push_str(&format!(" ({})", attrs.join(" ")));
        }

        // Textinhalt (direkte Text-Kindknoten sammeln)
        let text_content: String = node
            .children()
            .filter(|c| c.is_text())
            .filter_map(|c| c.text())
            .map(|t| t.trim())
            .collect();

        if !text_content.is_empty() {
            s.push_str(&format!(": {}", text_content.yellow()));
        }
        display_str = s;
    } else if node.is_comment() {
        if let Some(text) = node.text() {
            display_str = format!("# {}", text.trim()).dimmed().italic().to_string();
        }
    }

    // Knoten ausgeben (Wurzelknoten selbst wird übersprungen)
    if !display_str.is_empty() {
        println!("{}{}{}", prefix, marker, display_str);
    }

    // Nur Elemente und Kommentare für die Rekursion filtern
    let children: Vec<Node> = node
        .children()
        .filter(|c| c.is_element() || c.is_comment())
        .collect();

    let count = children.len();
    let new_prefix = format!("{}{}", prefix, child_prefix);

    for (i, child) in children.iter().enumerate() {
        print_tree(*child, &new_prefix, i == count - 1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: xmlv <datei.xml>");
        std::process::exit(1);
    }

    let text = fs::read_to_string(&args[1]).expect("Fehler beim Lesen der Datei");
    let doc = Document::parse(&text).expect("Fehler beim Parsen der XML-Datei");

    println!("{}", args[1].magenta().bold());
    print_tree(doc.root(), "", true);
}
