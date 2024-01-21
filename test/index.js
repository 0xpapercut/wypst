import katex from '../src/katex';
import wypst from 'wypst';
import { deleteFields } from './utils';

let renderDiv = document.getElementById('render');

function katexRender() {
    let katexDiv = document.getElementById('katex');

    let input = katexDiv.querySelector('#input');
    let output = katexDiv.querySelector('#output');

    input.addEventListener('input', function() {
        // Print KaTeX parse tree
        try {
            let tree = katex.parseTree(input.value, new katex.Settings({displayMode: true, strict: "ignore"}));
            deleteFields(tree, ['loc']);

            let treeHTML = prettyJsonToHtml(tree);
            output.innerHTML = treeHTML;
        } catch (error) {
            output.innerHTML = error;
        }

        // Render the equation
        try {
            katex.render(input.value, renderDiv, {displayMode: true});
        } catch (error) { console.log(error); }
    });
}
katexRender();

async function wypstRender() {
    let wypstDiv = document.getElementById('wypst');
    let typstDiv = document.getElementById('typst');

    let input = wypstDiv.querySelector('#input');
    let wypstOutput = wypstDiv.querySelector('#output');
    let typstOutput = typstDiv.querySelector('#output');

    await wypst.init();
    input.addEventListener('input', async function() {
        // Print Typst parse tree (in KaTeX representation)
        try {
            let tree = wypst.parseTree(input.value);
            let treeHTML = prettyJsonToHtml(tree);

            wypstOutput.innerHTML = treeHTML;
        } catch (error) {
            wypstOutput.innerHTML = error;
        }

        // Print Typst content tree
        try {
            let tree = wypst.__typstContentTree(input.value);
            let treeHTML = prettyStringToHtml(tree);
            typstOutput.innerHTML = treeHTML;
        } catch (error) {
            typstOutput.innerHTML = '';
         }

        // Render the equation
        try {
            wypst.render(input.value, renderDiv, {displayMode: true});
        } catch (error) { }
    });
}
wypstRender()

function prettyJsonToHtml(json) {
    let s = JSON.stringify(json, null, 4);
    return prettyStringToHtml(s);
}

function prettyStringToHtml(s) {
    return s.replace(/\n/g, '<br>').replace(/ /g, '&nbsp;');
}
