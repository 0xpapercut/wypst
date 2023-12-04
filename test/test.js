const chai = require('chai');
import { parseTree as coreParseTree, loadTypst } from '../src/lib';
import Settings from 'katex/src/Settings.js';
import parseTree from "katex/src/parseTree";
import { deleteFields } from '../src/utils';

let exprs = [
    ["x + y", "x + y"],
    ["(x + y)", "\\left(x + y\\right)"],
];

loadTypst().then(core => {
    for (let expr of exprs) {
        let coreTree = coreParseTree(expr[0]);
        let tree = parseTree(expr[1], new Settings({}));

        deleteFields(coreTree, ['loc']);
        deleteFields(tree, ['loc']);

        let fmt = `Expected coreTree to deep equal tree. \nTypstSyntaxTree: ${core.typst_tree_str(expr[0])}\nCoreTree: ${JSON.stringify(coreTree, null, 2)} \nTree: ${JSON.stringify(tree, null, 2)}`
        chai.expect(coreTree).to.deep.equal(tree, fmt);
    }
});
