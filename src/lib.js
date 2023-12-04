// reference: katex.js

import { buildTree } from 'katex/src/buildTree.js';
import ParseError from "katex/src/ParseError";
import Settings from 'katex/src/Settings.js';
import buildCommon from "katex/src/buildCommon";
import { SymbolNode } from 'katex/src/domTree';

var core;

function renderError(error, expression) {
    if (!(error instanceof ParseError)) {
        throw error;
    }
    const node = buildCommon.makeSpan(["katex-error"],
        [new SymbolNode(expression)]);
    node.setAttribute("title", error.toString());
    return node;
};

function renderToDomTree(expression) {
    let settings = new Settings({});
    try {
        const tree = parseTree(expression);
        return buildTree(tree, expression, settings);
    } catch (error) {
        return renderError(error, expression, settings);
    }
};

export function parseTree(expression) {
    if (!isTypstLoaded()) {
        throw new Error('You must first call loadTypst().');
    }
    return core.parse_tree(expression);
}

export function renderToString(expression) {
    const markup = renderToDomTree(expression).toMarkup();
    return markup;
};

export function render(expression, baseNode) {
    baseNode.textContent = "";
    const node = renderToDomTree(expression).toNode();
    baseNode.appendChild(node);
};

export async function loadTypst() {
    core = await import('./core/pkg');
    return core;
}

export function isTypstLoaded() {
    return !!core;
}

if (process.env.NODE_ENV === 'development') {
    require('./debug.js');
}

export { core }; // DEBUG PURPOSES
