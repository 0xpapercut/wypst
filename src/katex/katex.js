import katex from 'katex';
import Settings from 'katex/src/Settings';
import parseTree from 'katex/src/parseTree';
import buildTree from 'katex/src/buildTree';

export default {
    render: katex.render,
    renderToString: katex.renderToString,
    ParseError: katex.ParseError,
    Settings,
    parseTree,
    buildTree,
};
