import init, { parseTree as _parseTree, typstContentTree } from './core';
import utils from './utils';

function parseTree(expression, settings) {
    expression = expression.trim().replace(/\n/g, ' ');
    return _parseTree(expression, settings);
}

function renderToDomTree(expression, options) {
    let settings = new utils.Settings(options);
    try {
        const tree = parseTree(expression, settings);
        return utils.buildTree(tree, expression, settings);
    } catch (error) {
        // Temporary fix so that we actually see errors like "unknown variable: ..."
        return utils.renderError(error, error, settings);
    }
}

/**
 * Renders a Typst expression into the specified DOM element
 * @param expression A Typst expression
 * @param element The DOM element to render into
 * @param options Render options
 */
function render(expression, baseNode, options) {
    baseNode.textContent = "";
    const node = renderToDomTree(expression, options).toNode();
    baseNode.appendChild(node);
};

/**
 * Renders a Typst expression into an HTML string
 * @param expression A Typst expression
 * @param options Render options
 */
function renderToString(expression, options) {
    const markup = renderToDomTree(expression, options).toMarkup();
    return markup;
}

export default {
    render,
    renderToString,
    parseTree,
    init,
    __typstContentTree: typstContentTree,
};
