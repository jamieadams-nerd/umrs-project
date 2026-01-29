3. How It Works (Commands)

Build all examples

cargo build --examples

This compiles everything under examples/.

⸻

Run a specific example

cargo run --example labels_basic

Cargo:
	•	Compiles it (if needed)
	•	Links it against your library crate
	•	Runs it

No main.rs wiring needed elsewhere.

⸻

List all available examples

cargo run --example

Cargo will print:

error: "--example" takes one argument.
Available examples:
    labels_basic
    labels_validate
    time_probe
    crypto_posture


⸻

4. How Example Code Looks

Each file is just a normal Rust program with a main():

examples/labels_basic.rs

use umrs_core::labels;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = labels::parse("CUI//PRIVACY//FOUO")?;
    println!("{:#?}", parsed);
    Ok(())
}




7. Optional: Naming & Organization Discipline (Recommended for You)

Given your documentation rigor, I strongly recommend a convention like:

examples/
  labels/
    parse_basic.rs
    parse_strict.rs
    render_formats.rs

  logspace/
    scan.rs
    summarize.rs
    enforce_limits.rs

  crypto/
    fips_detect.rs
    provider_lockdown.rs


    8. When to Promote an Example into src/bin/

A clean lifecycle model for UMRS:
	1.	Prototype behavior in:

examples/crypto/fips_detect.rs


	2.	Document it, stabilize output, refine UX.
	3.	Promote into:

src/bin/umrs-fips.rs


	4.	Add CLI parsing (clap), logging, and policy wiring.

That keeps your production surface area clean and intentional.

⸻


