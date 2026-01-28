


⸻

How to run it

Build:

cargo build --release

Run with a message:

./target/release/umrs-matrix --seconds 8 --message "UMRS NOTICE\n\nTransfer complete.\n\nPress any key to exit."

Run with a message file:

./target/release/umrs-matrix --seconds 10 --message-file notice.txt

Run with stdin:

printf "UMRS NOTICE\n\nHello from stdin\n" | ./target/release/umrs-matrix --seconds 6

If you paste your original matrix-rain code, I can also merge these changes into your exact structure (keeping your existing RNG, glyph choices, frame loop, etc.).

Yes — effectively it’s black, but there’s a small technical nuance worth knowing.

What’s happening now
	•	The program clears the screen with Clear(ClearType::All)
	•	It does not set a background color explicitly
	•	That means the background is whatever your terminal’s default background is

For almost all terminals running a Matrix-style demo, that default is black, so visually:
	•	background = black
	•	glyphs = green/white
	•	box = white

If you want to guarantee black (explicitly)

If you want to be absolutely explicit and not depend on terminal defaults, add this once after entering the alternate screen:

execute!(
    stdout,
    SetBackgroundColor(Color::Black),
    Clear(ClearType::All)
)?;

And remember to reset colors on exit (which the code already does):

execute!(stdout, style::ResetColor)?;

Why this distinction matters
	•	Leaving it implicit respects the user’s terminal theme
	•	Explicit black guarantees the classic Matrix look
	•	For demos, alerts, or “cinematic” output → explicit black is usually preferable

Bottom line
	•	Visually: yes, it’s black
	•	Technically: it uses the terminal’s background unless you force it
	•	Recommendation for your use case: explicitly set black once, so it’s deterministic

If you want, I can give you the exact two-line patch to make black mandatory and still restore the user’s colors perfectly on exit.






USAGE:
  matrix_box [--seconds N] [--message "TEXT"] [--message-file PATH]

NOTES:
  - If stdin is piped, the message is read from stdin.
  - --message supports \n sequences for line breaks.
  - Press any key to exit early.

EXAMPLES:
  matrix_box --seconds 6 --message "UMRS NOTICE\n\nTransfer complete."
  echo -e "UMRS\n\nHello from stdin" | matrix_box --seconds 5
  matrix_box --message-file notice.txt --seconds 10



