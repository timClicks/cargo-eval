// #![deny(unsafe_code, missing_debug_implementations, missing_docs)]

use std::vec;
use std::process::Command;
// use std::str::FromStr;
use tempfile::Builder;
use std::fs::{self, File};
use std::io::{self, Write};


// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Dependency {
//     name: String,
//     version: String,
// }

// impl Dependency {
//     fn new<T: AsRef<str>>(name: T, version: T) -> Self {
//         Dependency {
//             name: name.as_ref().to_string(),
//             version: version.as_ref().to_string(),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum DependencyError {
//     NoName,
// }

// impl FromStr for Dependency {
//     type Err = DependencyError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut parts = s.trim().split(':');
        
//         let name = if let Some(name) = parts.next() {
//             name
//         } else {
//             return Err(DependencyError::NoName);
//         };

//         let version = parts.next().unwrap_or("*");

//         Ok(Dependency::new(
//             name,
//             version,
//         ))
//     }
// }



fn main() -> Result<(), io::Error>  {
    let args= std::env::args().collect::<Vec<_>>();

    let mut arg_parts = args.split(|arg| arg == "--");
    let _ = arg_parts.next();
    let default = vec![]; // TODO: return error
    let source = arg_parts.next().unwrap_or(&default).join(" ");
    println!("> {}", &source);

    // TODO: enable dependencies to be specified as command-line args
    // let mut dependencies: Vec<Dependency> = vec![];

    let dir = Builder::new()
        .prefix("rust-")
        .tempdir()?;
    
    let cargo_toml_path = dir.path().join("Cargo.toml");
    let mut cargo_toml = File::create(cargo_toml_path)?;
    write!(cargo_toml,
"[package]
name = \"temp\"
version = \"0.0.0\"
edition = \"2021\"

[dependencies]
")?;

    // for dep in dependencies {
    //     writeln!(cargo_toml, "{} = \"{}\"", dep.name, dep.version)?;
    // }

    let src_path = dir.path().join("src");
    fs::create_dir(src_path)?;

    let bin_path = dir.path().join("src").join("main.rs");
    let mut bin = File::create(bin_path)?;

    // let source = String::from(" 1 + 1 ");

    let has_main = source.contains("fn main");

    if !has_main {
        writeln!(bin, "fn main() -> Result<(), ()> {{")?;
        writeln!(bin, "let input = ")?;
    }

    writeln!(bin, "{}", source)?;

    if !has_main {
        writeln!(bin, ";")?;
        writeln!(bin, "println!(\"{{}}\", input);")?;

        writeln!(bin, "Ok(())\n}}")?;
    }

    let _ = Command::new("cargo")
        .arg("fmt")
        .current_dir(&dir)
        .output(); // ignore errors from rustfmt


    let cmd = Command::new("cargo")
        .current_dir(&dir)
        .arg("run")
        .output()
        .expect("unable to compile input");

    println!("{}", String::from_utf8_lossy(cmd.stdout.as_slice()));

    Ok(())
}

// #[cfg(test)]
// mod test_parsing {
//     use super::*;

//     #[test]
//     fn version() {
//         let expect = Dependency {
//             name: String::from("abc"),
//             version: String::from("*"),
//         };
//         let dep = "abc".parse::<Dependency>().unwrap();

//         assert_eq!(dep, expect)
//     }
// }
