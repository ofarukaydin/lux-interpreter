use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

fn define_ast(output_dir: &str, basename: &str, token_types: Vec<String>) -> io::Result<()> {
    let path = Path::new(output_dir).join("src").join(format!(
        "{}{}",
        basename.to_ascii_lowercase(),
        ".rs"
    ));

    println!("{:?}", path);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "use crate::token::{{Token, TokenLiteral}};");
    writeln!(writer, "pub trait {} {{", basename)?;
    writeln!(writer, "}}")?;

    for token in &token_types {
        let struct_name = token.split(';').nth(0).unwrap().trim();
        let field_names = token.split(';').nth(1).unwrap().trim();

        writeln!(writer, "pub struct {} {{", struct_name)?;
        writeln!(writer, "{}", field_names)?;
        writeln!(writer, "}}")?;
        writeln!(writer, "impl {} for {} {{ }}", basename, struct_name)?;
    }

    Ok(())
}

pub fn run_generator() {
    let output_dir = "./";
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   ; left: Option<Box<Expr>>, right: Option<Box<Expr>>, operator: Token"
                .to_string(),
            "Grouping ; expression: Option<Box<Expr>>".to_string(),
            "Literal  ; value: TokenLiteral".to_string(),
            "Unary    ; operator: Token, right: Option<Box<Expr>>".to_string(),
        ],
    )
    .expect("Failed to write to file");
}
