# Uber-urgent (not really)
- ~~Syntax highlighting in Tree-sitter~~

# Urgent
- ~~Finish the parser~~
- Semantic analysis
- Compilation/JIT/interpreting or all of the above e.g., Cranelift
- Proper string formatting like char formatting

# Not-as-urgent
- ~~Converting parse errors to `crate::Error` for pretty-printing.~~
- Stage 2 errors: match against the grammar rule names and handle the `expected` fields in
    `ParseError`.
- Update README to reflect the current design decisions and terminology.
- Macros? Probably quite difficult to do in LR(1), so I think a separate file to contain macros
    might be a good idea. However, this would make them seem like a separate part of the code which
    is probably a bad idea... I could also go with something like Rust's new "macro 2.0" system and
    use a `macro` keyword or something like that (thee `macro_rules!` thing kinda sucks).
    Proc-macros are really cool though.
- Give parameter warnings for anon function parameters being non snake_case
- Error about methods not being in an interface, impl block, or object creation
