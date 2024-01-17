import katexParseTree from 'katex/src/parseTree';
import katex from 'katex';
import wypst from 'wypst';
import { deleteFields } from './utils';

let renderDiv = document.getElementById('render');

function katexRender() {
    let katexDiv = document.getElementById('katex');

    let input = katexDiv.querySelector('#input');
    let output = katexDiv.querySelector('#output');

    input.addEventListener('input', function() {
        console.log(input.value);
        try {
            let tree = katexParseTree(input.value, {displayMode: true});
            deleteFields(tree, ['loc']);
            tree = JSON.stringify(tree, null, 2);
            tree = tree.replace(/\n/g, '<br>').replace(/ /g, '&nbsp;');
            output.innerHTML = tree;
        } catch (error) {
            output.innerHTML = error;
        }

        try {
            katex.render(input.value, renderDiv, {displayMode: true});
        } catch (error) { }
    });
}
katexRender();

function wypstRender() {
    console.log(wypst);
    console.log(wypst.renderToString("x"));
    // console.log(hello)
    // hello();
    // console.log(parseTree);
    // console.log(parseTree('x'));
    // await wypst.loadTypst();
    // let wypstDiv = document.getElementById('wypst');

    // let input = wypstDiv.querySelector('#input');
    // let output = wypstDiv.querySelector('#output');

    // input.addEventListener('input', function() {
    //     console.log(input.value);
    //     wypst.parseTree(input.value);

        // try {
            // let tree = wypst.parseTree(input.value);
            // tree = JSON.stringify(tree, null, 2);
            // tree = tree.replace(/\n/g, '<br>').replace(/ /g, '&nbsp;');
            // output.innerHTML = tree;
        // } catch (error) {
            // console.log(error);
        // }

        // try {
        //     wypst.render(input.value, renderDiv, {displayMode: true});
        // } catch (error) { }
    // })
    // console.log(render);
    // console.log(wypst.render);
    // let wypstDiv = document.getElementById('wypst');
    // let typstDiv = document.getElementById('typst');

    // let input = wypstDiv.querySelector('#input');

    // console.log(loadTypst);
    // loadTypst().then(x => console.log(x));
}
wypstRender()
