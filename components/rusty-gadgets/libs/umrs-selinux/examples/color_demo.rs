// SPDX-License-Identifier: MIT
//
// UMRS — Color + Translator Demonstration
//
// Demonstrates:
//
//  • Forward translator lookup  (range → marking)
//  • Reverse translator lookup  (marking → ranges)
//  • File context retrieval     (get_file_context)
//  • PID context retrieval      (get_pid_context)
//  • secolor.conf resolution (components → [SeColor; 4])
//

use std::path::Path;
use std::str::FromStr;

use umrs_selinux::context::SecurityContext;
use umrs_selinux::mcs::colors::{ContextComponents, load_default, resolve_colors};
use umrs_selinux::mcs::translator::{self, GLOBAL_TRANSLATOR, SecurityRange};
use umrs_selinux::utils::{get_file_context, get_pid_context};

pub fn ansi_rgb(fg: (u8, u8, u8), bg: (u8, u8, u8), text: &str) -> String {
    format!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
        fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, text
    )
}

fn show_colors_for_ctx(ctx: &SecurityContext) {
    // Resolve MLS range string safely
    let range_str = ctx.level().map(|lvl| lvl.raw()).unwrap_or("s0");

    // Build component view for color resolution
    let comps = ContextComponents {
        user: ctx.user().as_str(),
        role: ctx.role().as_str(),
        r#type: ctx.security_type().as_str(),
        range: range_str,
    };

    // Resolve colors
    let colors = resolve_colors(&comps, &load_default().expect("secolor load failed"));

    // ANSI helper
    fn ansi_rgb(fg: (u8, u8, u8), bg: (u8, u8, u8), text: &str) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, text
        )
    }

    // Build colored context string (4 components ONLY)
    let user = ctx.user().as_str();
    let role = ctx.role().as_str();
    let ty = ctx.security_type().as_str();

    let colored = format!(
        "{}:{}:{}:{}",
        ansi_rgb(
            (colors[0].fg.r, colors[0].fg.g, colors[0].fg.b),
            (colors[0].bg.r, colors[0].bg.g, colors[0].bg.b),
            user
        ),
        ansi_rgb(
            (colors[1].fg.r, colors[1].fg.g, colors[1].fg.b),
            (colors[1].bg.r, colors[1].bg.g, colors[1].bg.b),
            role
        ),
        ansi_rgb(
            (colors[2].fg.r, colors[2].fg.g, colors[2].fg.b),
            (colors[2].bg.r, colors[2].bg.g, colors[2].bg.b),
            ty
        ),
        ansi_rgb(
            (colors[3].fg.r, colors[3].fg.g, colors[3].fg.b),
            (colors[3].bg.r, colors[3].bg.g, colors[3].bg.b),
            range_str
        ),
    );

    println!("Colored: {}", colored);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    translator::load_setrans_file("data/setrans.conf")?;
    let _guard = GLOBAL_TRANSLATOR.read().unwrap();

    // ---------------------------------------------------------------------
    // Load secolor.conf
    // ---------------------------------------------------------------------
    let colors_cfg = load_default()?;
    println!("Loaded secolor.conf\n");

    println!("\n=== UMRS Color Demonstration ===\n");

    // Local helper: print the 4 resolved colors from ContextComponents
    let print_colors = |comps: &ContextComponents| {
        let colors = resolve_colors(comps, &colors_cfg);

        let labels = ["USER", "ROLE", "TYPE", "RANGE"];
        for (i, c) in colors.iter().enumerate() {
            println!(
                "  {:<6} FG #{:02x}{:02x}{:02x}  BG #{:02x}{:02x}{:02x}",
                labels[i], c.fg.r, c.fg.g, c.fg.b, c.bg.r, c.bg.g, c.bg.b
            );
        }
    };

    // ---------------------------------------------------------------------
    // Forward Lookup Demo
    // ---------------------------------------------------------------------
    println!("--- Forward Lookup ---");

    let query = "s0:c90,c91";
    let range = SecurityRange::from_str(query)?;

    let translator = GLOBAL_TRANSLATOR.read().expect("Translator lock poisoned");

    let _marking = translator.lookup(&range).unwrap_or_else(|| "MISSING".to_string());

    //println!("Range  : {query}");
    ////println!("Marking: {marking}");
    // ---------------------------------------------------------------------
    // Reverse Lookup Demo (Marking -> Kernel ranges -> Colors)
    // ---------------------------------------------------------------------
    println!("\n--- Reverse Lookup ---");

    let search = "CUI//LEI/AIV";
    //let search = "THIS_DOES_NOT_EXIST";

    println!("SEARCH = [{search}]");

    let results = translator.lookup_by_marking(search);

    if results.is_empty() {
        println!("No ranges found for {search}");
    } else {
        for (kernel_str, detail) in results {
            if detail.is_empty() {
                println!("{search} -> {kernel_str}");
            } else {
                println!("{search} -> {kernel_str}  # {detail}");
            }

            // secolor.conf RANGE rules match on the raw range string (e.g. "s0:c90.c107")
            // so we synthesize components and feed them to resolve_colors().
            let comps = ContextComponents {
                user: "system_u",
                role: "object_r",
                r#type: "default_t",
                range: kernel_str.as_str(),
            };

            println!("Resolved Colors (synthetic components):");
            print_colors(&comps);
        }
    }

    // ---------------------------------------------------------------------
    // File Context Demo
    // ---------------------------------------------------------------------
    println!("\n--- File Context Demo ---");

    let path = Path::new("/etc/passwd");
    let file_ctx: SecurityContext = get_file_context(path)?;

    println!("File: {}", path.display());
    println!("Context: {file_ctx}");
    show_colors_for_ctx(&file_ctx);

    // Second test
    println!("\n");
    let path = Path::new("./TESTFILE");
    let file_ctx: SecurityContext = get_file_context(path)?;

    println!("File: {}", path.display());
    println!("Context: {file_ctx}");
    show_colors_for_ctx(&file_ctx);

    // ---------------------------------------------------------------------
    // PID Context Demo
    // ---------------------------------------------------------------------
    println!("\n--- PID Context Demo ---");

    let pid = std::process::id();
    let pid_ctx = get_pid_context(pid)?;

    println!("PID: {pid}");
    println!("Context: {pid_ctx}");
    show_colors_for_ctx(&pid_ctx);

    Ok(())
}
