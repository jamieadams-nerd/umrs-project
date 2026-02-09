use std::env;
use std::fs;
use std::path::Path;

use umrs_core::robots::{AsciiArt, build_robot_art};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "usage: {} <robot-name> <ascii-file>",
            args[0]
        );
        std::process::exit(1);
    }

    let name = &args[1];
    let path = Path::new(&args[2]);

    let input = fs::read_to_string(path)?;
    let robot = build_robot_art(name, &input)?;

    emit_robot_static(&robot);

    Ok(())
}

fn emit_robot_static(robot: &AsciiArt) {
    let ident = robot.name.to_uppercase();

    println!("pub static {}: AsciiArtStatic = AsciiArtStatic {{", ident);
    println!("    name: {:?},", robot.name);
    println!("    width: {},", robot.width);
    println!("    height: {},", robot.height);
    println!("    lines: &[");

    for line in &robot.lines {
        println!("        {:?},", line);
    }

    println!("    ],");
    println!("}};");
}




