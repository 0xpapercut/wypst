# wypst
Typst math typesetting for the web.

## Usage
You can load this library either by using a script tag, or installing it with npm.

### Script tag (simple usage)
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/wypst@0.0.8/dist/wypst.min.css" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/wypst@0.0.8/dist/wypst.min.js" crossorigin="anonymous"></script>

<script>
    wypst.initialize().then(() => {
        wypst.renderToString("x + y"); // Test it out!
    })
</script>
```

Keep in mind that the javascript file is 17M, so if your internet is slow it might take some seconds to load.

### npm package (advanced usage)
If having the wasm inlined directly is an incovenience, install the npm package
```bash
npm install wypst
```

You may then load the wasm binary
```javascript
import wypst from 'wypst';
import wasm from 'wypst/dist/wypst.wasm';

await wypst.initialize(wasm);
wypst.renderToString("x + y"); // Test it out!
```

Keep in mind that you will probably need to tell your bundler how to load a `.wasm` file. If you have difficulties you can open an issue.

### Rendering Typst Math
To render a Typst math expression, you can use either `render` or `renderToString`, as the example below shows:
```javascript
wypst.render('sum_(n >= 1) 1/n^2 = pi^2/6', element); // Renders into the HTML element
wypst.renderToString('sum_(n >= 1) 1/n^2 = pi^2/6'); // Renders into an HTML string
```

## Contributing
All help is welcome. Please see [CONTRIBUTING](CONTRIBUTING.md).
