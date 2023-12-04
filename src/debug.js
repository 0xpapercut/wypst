import Settings from "katex/src/Settings";
import katexParseTree from "katex/src/parseTree";
import { buildTree } from 'katex/src/buildTree.js';
import { deleteFields } from './utils'
import { loadTypst } from "./lib";

document.getElementById("katex-input").addEventListener('input', function() {
    try {
        let tree = katexParseTree(this.value, new Settings({}));
        deleteFields(tree, ['loc']);
        let treeStr = treeString(tree);
        document.getElementById("katex-output").innerHTML = treeStr;
        render(tree, '');
    } catch (e) {
        document.getElementById("katex-output").innerHTML = e;
    }
});

document.getElementById("typst-input").addEventListener('input', async function() {
    let tree = await typstParseTreeString(this.value);
    document.getElementById("typst-output").innerHTML = tree;
});

document.getElementById("wypst-input").addEventListener('input', async function() {
    let tree = await katexFromTypstParseTree(this.value);
    deleteFields(tree, ['loc']);
    let treeStr = treeString(tree);
    document.getElementById("wypst-output").innerHTML = treeStr;
    render(tree, '');
});

async function typstParseTreeString(input) {
    if (!input)
        return {};
    let core = await loadTypst();
    let tree = core.typst_tree_str(input);
    return tree.replace(/\n/g, '<br>').replace(/ /g, '&nbsp;');
}

async function katexFromTypstParseTree(expression) {
    let core = await loadTypst();
    let tree = core.parse_tree(expression);
    return tree;
}

function treeString(tree) {
    tree = JSON.stringify(tree, null, 2);
    return tree.replace(/\n/g, '<br>').replace(/ /g, '&nbsp;');
}

function render(tree, expression) {
    let renderDiv = document.getElementById("render");
    // try {
        let node = buildTree(tree, expression, new Settings({})).toNode();
        renderDiv.innerHTML = '';
        renderDiv.appendChild(node);
    // } catch (e) {
        // renderDiv.innerHTML = e;
    // }
}

import '../test/test';
