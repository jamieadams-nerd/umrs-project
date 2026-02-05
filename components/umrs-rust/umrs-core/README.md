

Usage: cargo run --example [<NAME>] [ARGS]...

For more information, try '--help'.
[git ⎇ main][umrs-project] /src/umrs-rust/umrs-core ➜ cargo run --example size_format
warning: hard linking files in the incremental compilation cache failed. copying files instead. consider moving the cache directory to a file system which supports hard linking in session dir `/Volumes/LaCie/Development/repos/umrs-project/src/umrs-rust/target/debug/incremental/umrs_core-12aypmt6bd8cm/s-hfb619so2f-0k67qfj-working`

warning: `umrs-core` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `/Volumes/LaCie/Development/repos/umrs-project/src/umrs-rust/target/debug/examples/size_format`
Auto (SI):  1.50 MB
Auto (IEC): 1.43 MiB
Forced MB:  1.50 MB
Forced MiB: 1.43 MiB
Forced GB:  0.00 GB




## Timed Results

✅ EXAMPLE 1 — FUNCTION THAT CANNOT FAIL

Existing domain type

#[derive(Debug)]
struct SystemState {
    version: String,
    healthy: bool,
}

Function

use crate::core::timed::Timed;

fn get_system_state() -> Timed<SystemState> {
    Timed::measure(|| {
        SystemState {
            version: "1.2.3".to_string(),
            healthy: true,
        }
    })
}

Caller

let state = get_system_state();

println!("Elapsed: {:?}", state.elapsed());
println!("Version: {}", state.value.version);
println!("Healthy: {}", state.value.healthy);


⸻

✅ EXAMPLE 2 — FUNCTION THAT MAY FAIL

Domain type

#[derive(Debug)]
struct Person {
    name: String,
}

Function

use crate::core::timed::TimedResult;

fn get_person(id: u64) -> TimedResult<Person, &'static str> {
    TimedResult::measure(|| {
        if id == 42 {
            Ok(Person {
                name: "Alice".to_string(),
            })
        } else {
            Err("person not found")
        }
    })
}

Caller (success or failure handled the same way)

let result = get_person(42);

println!("Elapsed: {:?}", result.elapsed());

match result.value {
    Ok(person) => println!("Found person: {}", person.name),
    Err(e) => println!("Error: {}", e),
}


⸻

✅ EXAMPLE 3 — GENERIC CODE USING THE TRAIT

This is where the formal Rust design pays off.

use crate::core::timed::HasElapsed;

fn log_timing<T: HasElapsed>(t: &T) {
    println!("Operation took {:?}", t.elapsed());
}

Works for both:

log_timing(&get_system_state());
log_timing(&get_person(7));

No generics explosion.
No enums.
No hacks.

⸻





Textwrap and Boxmsg modules

4. Putting It All Together (Wrap → Box → Print)

Here’s the full pipeline you were intuitively heading toward:

use textwrap::{Options, wrap_preserve_newlines};

fn wrap_and_box(input: &str, width: usize, indent: usize) -> String {
    let indent_str = " ".repeat(indent);

    let options = Options::new(width)
        .initial_indent(&indent_str)
        .subsequent_indent(&indent_str)
        .break_words(false);

    let lines = wrap_preserve_newlines(input, &options);

    box_lines(&lines, 1, &BoxStyle::UNICODE)
}

Usage:

let text = r#"
This is a long paragraph that should be wrapped nicely to a fixed width
and then placed into a Unicode text box for terminal display. It should
preserve paragraph breaks and avoid hyphenation.

This is the second paragraph.
"#;

println!("{}", wrap_and_box(text, 40, 0));

Output:

┌──────────────────────────────────────────┐
│ This is a long paragraph that should be  │
│ wrapped nicely to a fixed width and then │
│ placed into a Unicode text box for       │
│ terminal display. It should preserve     │
│ paragraph breaks and avoid hyphenation.  │
│                                          │
│ This is the second paragraph.            │
└──────────────────────────────────────────┘


⸻

