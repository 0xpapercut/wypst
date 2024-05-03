import ParseError from '../lib/katex/src/ParseError';
import Settings from '../lib/katex/src/Settings';
import parseTree from '../lib/katex/src/parseTree';
import buildTree from '../lib/katex/src/buildTree';
import buildCommon from '../lib/katex/src/buildCommon';
import { SymbolNode } from '../lib/katex/src/domTree';

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
