# wypst
Typst math typesetting for the web.

## Getting started
### Installation
Install wypst with npm:
```bash
npm install wypst
```
### Importing
You can import wypst as an ECMAScript module:
```javascript
import wypst from 'wypst';
```
### Rendering Typst Math
To render a Typst math expression, you can use either `render` or `renderToString`, as the example below shows:
```javascript
wypst.render('sum_(n >= 1) 1/n^2 = pi^2/6', element); // Renders into the HTML element
wypst.renderToString('sum_(n >= 1) 1/n^2 = pi^2/6'); // Renders into an HTML string
```

## Contributing
All help is welcome. Please see [CONTRIBUTING](CONTRIBUTING.md).

## Disclaimer
As of now wypst is very experimental, so you may encounter lots of bugs. I expect to have most fixed by mid March.
