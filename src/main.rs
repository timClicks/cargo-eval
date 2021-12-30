// #![deny(unsafe_code, missing_debug_implementations, missing_docs)]

use std::fmt;
use std::process::Command;
use std::str::FromStr;
use std::fs::{self, File};
use std::io::{self, Write, Read};

use structopt::{self, StructOpt};
use tempfile::Builder;


#[derive(Debug, Clone, PartialEq, Eq)]
struct Dependency {
    name: String,
    version: String,
}

impl Dependency {
    fn new<T: AsRef<str>>(name: T, version: T) -> Self {
        Dependency {
            name: name.as_ref().to_string(),
            version: version.as_ref().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DependencyError {
    NoName,
}

impl fmt::Display for DependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyError::NoName => writeln!(f, "no dependency name provided")
        }
    }
}

impl FromStr for Dependency {
    type Err = DependencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(':');
        
        let name = if let Some(name) = parts.next() {
            name
        } else {
            return Err(DependencyError::NoName);
        };

        let version = parts.next().unwrap_or("*");

        Ok(Dependency::new(
            name,
            version,
        ))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo eval", about = "Evaluate Rust source code", rename_all = "kebab-case", max_term_width = 80)]
struct Options {
    /// Add dependency
    /// 
    /// Include a dependency to the program that will be executed
    /// by Rust. Can be provided multiple times.
    #[structopt(short, long = "dep")]
    dependencies: Vec<Dependency>,

    /// Increase verbosity
    /// 
    /// Prints the contents of the intermediate files that are 
    /// generated.
    #[structopt(short, long)]
    verbose: bool,
}

fn main() -> Result<(), io::Error>  {
    let env_args: Vec<_> = std::env::args()
        .collect();

    let mut args: Vec<_> = env_args
        .iter()
        .skip_while(|arg| **arg != "eval")
        .skip(1)
        .take_while(|arg| **arg != "--")
        .map(|x| x.clone())
        .collect();
    args.insert(0, env_args[0].clone());

    let opt = Options::from_iter(&args);

    let source = env_args
        .iter()
        .skip_while(|arg| **arg != "--")
        .skip(1)
        .map(|x| x.clone())
        .collect::<Vec<_>>()
        ;
    let source = source
        .join(" ")
        .trim()
        .to_string();

    // TODO: enable dependencies to be specified as command-line args

    let dir = Builder::new()
        .prefix("rust-")
        .tempdir()?;
    
    let cargo_toml_path = dir.path().join("Cargo.toml");
    let mut cargo_toml = File::create(&cargo_toml_path)?;
    write!(cargo_toml,
"[package]
name = \"temp\"
version = \"0.0.0\"
edition = \"2021\"

[dependencies]
")?;

    for dep in opt.dependencies {
        writeln!(cargo_toml, "{} = \"{}\"", dep.name, dep.version)?;
    }

    let src_path = dir.path().join("src");
    fs::create_dir(src_path)?;

    let bin_path = dir.path().join("src").join("main.rs");
    let mut bin = File::create(&bin_path)?;

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
    
    if opt.verbose {
        let mut f = File::open(cargo_toml_path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        println!("Cargo.toml\n{}", &buf);

        let mut f = File::open(&bin_path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        println!("src/main.rs\n{}", &buf);
    }

    let cmd = Command::new("cargo")
        .current_dir(&dir)
        .arg("run")
        .output()
        .expect("unable to compile input");
    
    println!("> {}", &source);
    print!("{}", String::from_utf8_lossy(cmd.stdout.as_slice()));

    Ok(())
}
