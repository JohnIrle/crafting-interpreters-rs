use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = &args[1];
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   ; left: Expr, operator: Token, right: Expr",
            "Grouping ; expression: Expr",
            "Literal  ; value: String",
            "Unary    ; operator: Token, right: Expr",
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> std::io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    dbg!(&path);
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "use crate::Token;")?;
    writeln!(file)?;
    for type_ in &types {
        let class_name = type_.split(';').collect::<Vec<&str>>()[0].trim();
        let fields = type_.split(';').collect::<Vec<&str>>()[1].trim();

        define_structs(&mut file, class_name, fields)?
    }
    writeln!(file, "pub enum {} {{", base_name)?;

    for type_ in &types {
        let class_name = type_.split(';').collect::<Vec<&str>>()[0].trim();

        define_type(&mut file, class_name)?;
    }

    writeln!(file, "}}")?;

    define_visitor(&mut file, base_name, types)?;
    Ok(())
}

fn define_structs(
    file: &mut std::fs::File,
    class_name: &str,
    field_list: &str,
) -> std::io::Result<()> {
    write!(file, "pub struct {} {{", class_name)?;
    let fields = field_list.split(", ").collect::<Vec<&str>>();
    for (i, mut field) in fields.iter().enumerate() {
        let replaced = field.replace("Expr", "Box<Expr>");
        if i == fields.len() - 1 {
            write!(file, "\n    pub {}\n", replaced)?;
        } else {
            write!(file, "\n    pub {},", replaced)?;
        }
    }
    writeln!(file, "}}")?;
    Ok(())
}

fn define_type(file: &mut std::fs::File, class_name: &str) -> std::io::Result<()> {
    write!(file, "    {}({}", class_name, class_name)?;
    writeln!(file, "),")?;
    Ok(())
}

fn define_visitor(
    file: &mut std::fs::File,
    base_name: &str,
    types: Vec<&str>,
) -> std::io::Result<()> {
    writeln!(file)?;
    writeln!(file, "pub trait Visitor<T> {{")?;

    for type_ in &types {
        let type_name = type_.split(';').collect::<Vec<&str>>()[0].trim();

        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> T;",
            type_name.to_lowercase(),
            base_name.to_lowercase(),
            type_name
        )?;
    }

    writeln!(file, "}}")?;

    writeln!(file)?;
    writeln!(file, "pub trait Accept<T> {{")?;
    writeln!(
        file,
        "    fn accept<V: Visitor<T>>(&self, visitor: &V) -> T;"
    )?;
    writeln!(file, "}}")?;

    writeln!(file)?;
    writeln!(file, "impl Accept<String> for {} {{", base_name)?;
    writeln!(
        file,
        "    fn accept<V: Visitor<String>>(&self, visitor: &V) -> String {{"
    )?;
    writeln!(file, "        match self {{")?;

    for type_ in &types {
        let type_name = type_.split(';').collect::<Vec<&str>>()[0].trim();

        write!(file, "            {}::{}(expr) =>", base_name, type_name,)?;
        writeln!(
            file,
            " visitor.visit_{}_{}(expr),",
            type_name.to_lowercase(),
            base_name.to_lowercase(),
        )?;
    }

    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    Ok(())
}
