# wypst
Typst math typesetting for the web.

## Usage
You can load this library either by using a script tag (preferred), or installing it with npm (for advanced usage).

### Option 1: Script tag
This is simplest way to load, and looks very similar to how you go about it with KaTeX. This is the preferred and simplest method.

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/wypst@0.0.5/dist/wypst.min.css" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/wypst@0.0.5/dist/wypst.min.js" crossorigin="anonymous"></script>
```

Keep in mind that the javascript file is 17M, so if your internet is slow it might take some seconds to load.

### Option 2: npm package
Sometimes having the wasm inlined in the script file is incovenient. When this is the case, you will have to install the package with npm.

```bash
npm install wypst
```

You may then load the wasm binary

```
import wypst from 'wypst';
import wasm from 'wypst/dist/wypst.wasm';

wypst.initialize(wasm);
wypst.renderToString("x + y") // Test it out!
```

### Rendering Typst Math
To render a Typst math expression, you can use either `render` or `renderToString`, as the example below shows:
```javascript
wypst.render('sum_(n >= 1) 1/n^2 = pi^2/6', element); // Renders into the HTML element
wypst.renderToString('sum_(n >= 1) 1/n^2 = pi^2/6'); // Renders into an HTML string
```

## Contributing
All help is welcome. Please see [CONTRIBUTING](CONTRIBUTING.md).
