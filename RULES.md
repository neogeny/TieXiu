---
apply: always
---

# Rules

## Interaction

- Never change files without a plan and user authorization
- Never change more than the files explicitly named in the authorization
- Always consult with the User before making changes that impact multiple files
- Evaluate changes before applying them (no "apply and see" approach)
- Do not act on assumptions. Always verify assumptions with the User.

## Testing

- Tests can be marked with `#[ignore]` but must compile
- Run tests with `cargo test --verbose`
- Use of `.unwrap()` or `.expect()` in tests is forbidden. Tests functions must
  return `tiexiu::Result<()>` and use the `?` notation to forward any errors
  to the test runner.

  For example:
    ```rust
    let tree = tiexiu::parse_grammar(grammar, &[])?;
    ```

## Code Tools

- Do not use `sed`, `awk`, or similar text tools to modify code
- When using `rg`, explicitly name the directories to search (e.g., `rg -l 'pattern' src tests`) instead of excluding directories
- Create `./tmp/` for temporary files instead of using `/tmp`
- Never try to access or modify a file or directory outside the current
  project's directory

## Code Style

- Instead of calling specific constructors like Box::new() or Rc::new(), the
  preference in this project is to use .into() whenever possible, as it hides
  decisions about typing and require no modification after a change in typing.
- Error Delegation: Utilize thiserror's #[from] attribute for submodule error promotion.
