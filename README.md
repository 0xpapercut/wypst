# wypst
Typst math typesetting for the web.

## Getting started
> TODO

## Known Issues
- `sum` is not represented correctly because it doesn't have `limits` set to `true`:
- Spaces around `∣` are eaten up, for example in `{x | x "is natural"}`. Also in the same expression, space between `x` and `"is natural"` is eatuen up as well. This problem extends to some other symbols as well, so any solution must be somewhat general.

## Disclaimer
wypst is not yet ready to be used. It is expected to be usable at start of february.

The focus is to provide a complete, working implementation for a subset of the math syntax of typst. When it is ready, this subset will be explicitly described.

Finally, the rendering process is expected to have equivalent speed to that of KaTeX. Once appropriate I'll share benchmarks.

## Working expressions
- A = pi r^2
