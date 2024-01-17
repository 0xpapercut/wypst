import { parseTree } from './src/core/pkg';
import katex from './src/katex';

function renderToDomTree(expression, options) {
    let settings = new katex.Settings({});
    try {
        const tree = parseTree(expression, settings);
        return katex.buildTree(tree, expression, settings);
    } catch (error) {
        return renderError(error, expression, settings);
    }
}

function renderError(error, expression, options) {
    // TODO
}

/**
 * Renders a Typst expression into the specified DOM element
 * @param expression A Typst expression
 * @param element The DOM element to render into
 * @param options Render options
 */
function render(expression, baseNode, options) {
    baseNode.textContent = "";
    const node = renderToDomTree(expression).toNode();
    baseNode.appendChild(node);
};

/**
 * Renders a Typst expression into an HTML string
 * @param expression A Typst expression
 * @param options Render options
 */
function renderToString(expression, options) {
    // renderToDomTree()
    // let x = new katex.Settings({});
    // return katex.renderToString(expression, options);
    const markup = renderToDomTree(expression, options).toMarkup();
    return markup;
}

export default {
    render,
    renderToString,
    parseTree,
};
