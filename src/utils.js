import ParseError from 'katex/src/ParseError';
import Settings from 'katex/src/Settings';
import parseTree from 'katex/src/parseTree';
import buildTree from 'katex/src/buildTree';
import buildCommon from 'katex/src/buildCommon';
import { SymbolNode } from 'katex/src/domTree';

function renderError(error, expression, settings) {
    if (settings.throwOnError) {
        throw error;
    }
    const node = buildCommon.makeSpan(["katex-error"], [new SymbolNode(expression)]);
    node.setAttribute("title", error.toString());
    node.setAttribute("style", `color:${settings.errorColor}`);
    return node;
}

export default {
    ParseError,
    Settings,
    parseTree,
    buildTree,
    renderError,
};
