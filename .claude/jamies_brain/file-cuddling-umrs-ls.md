# File Cuddling (Compact View)

Many files such as logs have been copied, rotated, or archived. The basename of the file stays the
same but is appleid a suffix. In the umrs-ls directory view, let's show the original base file, then
"roll-up" siblings wiht a count like "5 rolled up files" or something like that. 

NOTE: Keep in mind, soon the umrs-ls will be a TUI interface so these rows will be in table with
scrollable and selectable files. that means this rollup data must kept int he same row as the base
file. 

EXAMPLE FORMAT: If a file has a signture file, roll it up.
```
Filename.txt
Filename.txt.sig
```

In the compact/cuddle listing:
```
Filename.txt
  └ 1 signature file
```

For log files, 
```
boot.log
boot.log-20251210
boot.log-20251211
boot.log-20260125
boot.log-20260126
boot.log-20260302
boot.log-20260303
boot.log-20260310
```

Becomes:
```
boot.log
  └ 7 archives
```

- I don't know what the exact term should be. 
- Open it for duscission. 
- It might be nice to give count and aggregrte bytes consumed. 
- if it is clearly a log file (in log directory and has .log, then we could use "7 rotations"

## Identify Related files (rotated/copied)
For example, a log file that has been rotated. 


If the delimiter between the base filename and the rotation is not guaranteed (not always .), then the safest approach is:
	1.	Treat the first filename encountered as the base.
	2.	Count subsequent entries that start with the base name.
	3.	Only count entries that are longer than the base and look like rotations (optional numeric check).
	4.	Stop when the prefix no longer matches.

This keeps the logic fully generic and relies only on starts_with, which is exactly what you described.

### Generic Rust Implementation

fn main() {
    let files = vec![
        "file.log",
        "file.log.1",
        "file.log.2",
        "kernel_log",
        "kernel_log-1",
        "kernel_log-2",
        "system",
        "system.1",
        "system.2",
        "system.3",
    ];

    let mut i = 0;

    while i < files.len() {
        let base = &files[i];
        let mut rotations = 0;

        let mut j = i + 1;

        while j < files.len() {
            let candidate = &files[j];

            // Must start with the base name
            if !candidate.starts_with(base) {
                break;
            }

            // Must be longer than the base (otherwise identical name)
            if candidate.len() > base.len() {
                rotations += 1;
            }

            j += 1;
        }

        println!("{} with {} rotations", base, rotations);

        i = j;
    }
}


## Why This Approach Is Robust

This logic:
- does not depend on .
- works with
- file.log.1
- log-1
- file.log_1
- file.log.20240301
- relies purely on the sorted prefix grouping
- performs O(n) with no allocations


## Optional: Safer Rotation Detection

If you want to avoid false positives like:

file.log_backup
file.log_copy

you can require the suffix to begin with a separator.

Example check:

if let Some(rest) = candidate.strip_prefix(base) {
    if rest.starts_with('.') || rest.starts_with('-') || rest.starts_with('_') {
        rotations += 1;
    } else {
        break;
    }
}



## Small Performance Improvement (important for large log directories)

Avoid using format!() or temporary allocations. The above version already does this.

