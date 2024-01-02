# wypst
Typst math typesetting for the web.

## Disclaimer
wypst is not yet ready to be used. It is expected to be usable at start of february.

The focus is to provide a complete, working implementation for a subset of the math syntax of typst. When it is ready, this subset will be explicitly described.

Finally, the rendering process is expected to have equivalent speed to that of KaTeX. Once appropriate I'll share benchmarks.

## Issues
- `sum` is not represented correctly because it doesn't have `limits` set to `true`:
- The `:=` symbol is not rendered correctly.
- Spaces around `∣` are eaten up, for example in `{x | x "is natural"}`. This problem extends to some other symbols as well, so any solution must be somewhat general.
