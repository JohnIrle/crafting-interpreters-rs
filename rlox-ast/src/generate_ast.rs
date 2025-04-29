// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-ast

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

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
    )
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> std::io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    dbg!(&path);
    let mut file = std::fs::File::create(path)?;

    // Copyright and License header
    writeln!(
        file,
        "// SPDX-FileCopyrightText: 2024 John Irle\n// SPDX-License-Identifier: MIT\n//\n// This file is part of rlox-ast\n"
    )?;

    writeln!(file, "use crate::Token;")?;
    writeln!(file)?;
    writeln!(file, "pub enum {} {{", base_name)?;

    for type_ in &types {
        let (class_name, fields) = parse_type(type_);

        define_type(&mut file, class_name, &fields)?;
    }

    writeln!(file, "}}")?;

    define_visitor(&mut file, base_name, &types)?;

    Ok(())
}

fn define_type(
    file: &mut std::fs::File,
    class_name: &str,
    fields: &[(&str, &str)],
) -> std::io::Result<()> {
    let field_types: Vec<&str> = fields
        .iter()
        .map(|(_, typ)| if *typ == "Expr" { "Box<Expr>" } else { *typ })
        .collect();

    writeln!(file, "    {}({}),", class_name, field_types.join(", "))?;

    Ok(())
}

fn define_visitor(
    file: &mut std::fs::File,
    base_name: &str,
    types: &[&str],
) -> std::io::Result<()> {
    writeln!(file)?;
    writeln!(file, "pub trait Visitor<T> {{")?;

    for type_ in types {
        let (type_name, fields) = parse_type(type_);
        let args = fields
            .iter()
            .map(|(name, typ)| {
                if *typ == "String" {
                    format!("{}: &str", name)
                } else {
                    format!("{}: &{}", name, typ)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            file,
            "    fn visit_{}_{}(&self, {}) -> T;",
            type_name.to_lowercase(),
            base_name.to_lowercase(),
            args
        )?;
    }

    writeln!(file, "}}\n")?;

    writeln!(file, "pub trait Accept<T> {{")?;
    writeln!(
        file,
        "    fn accept<V: Visitor<T>>(&self, visitor: &V) -> T;"
    )?;
    writeln!(file, "}}\n")?;

    writeln!(file, "impl Accept<String> for {} {{", base_name)?;
    writeln!(
        file,
        "    fn accept<V: Visitor<String>>(&self, visitor: &V) -> String {{"
    )?;
    writeln!(file, "        match self {{")?;

    for type_ in types {
        let (type_name, fields) = parse_type(type_);
        let field_names = fields
            .iter()
            .map(|(n, _)| *n)
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            file,
            r#"            Self::{}({}) => "#,
            type_name, &field_names
        )?;
        writeln!(
            file,
            "visitor.visit_{}_{}({}),",
            type_name.to_lowercase(),
            base_name.to_lowercase(),
            field_names
        )?;
    }

    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    Ok(())
}

fn parse_type(type_def: &str) -> (&str, Vec<(&str, &str)>) {
    let parts: Vec<&str> = type_def.split(';').collect();
    let class_name = parts[0].trim();
    let fields = parts[1]
        .trim()
        .split(", ")
        .map(|f| {
            let parts = f.split(": ").collect::<Vec<_>>();
            (parts[0].trim(), parts[1].trim())
        })
        .collect();
    (class_name, fields)
}
