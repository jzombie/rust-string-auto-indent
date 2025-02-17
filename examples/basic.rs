use string_auto_indent::auto_indent;

fn main() {
    println!("");
    let text = r#"Example:
        A
            B
                C
    "#;

    println!("With auto-indent:");
    print!("{}", auto_indent(text));

    println!("-----");

    println!("Without auto-indent:");
    print!("{}", text);
    println!("");
}
