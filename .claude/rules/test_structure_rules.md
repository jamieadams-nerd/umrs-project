## Test Placement Rule

- Inline unit tests are prohibited.
- Do not use #[cfg(test)] in source files.
- Do not create mod tests inside src/.
- All tests must reside under each crate's `tests/` directory (e.g., `umrs-selinux/tests/`).
- Source files must contain production code only.
