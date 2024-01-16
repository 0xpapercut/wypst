const chai = require('chai');
import { parseTree as katexParseTree, loadTypst } from '../src/lib';
import Settings from 'katex/src/Settings.js';
import parseTree from "katex/src/parseTree";
import { deleteFields, deleteIfFieldEquals, diff } from '../src/utils';

let exprs = [
    // perfect
    ["x + y", "x + y"],
    ["(x + y)", "\\left(x + y\\right)"],
    ["cancel(x + y)", '\\cancel{x + y}'],
    ["cancel(x)", '\\cancel{x}'],

    // good, doesn't generate the same tree but generates the same render
    // ["A = pi r^2", "A = \\pi r^2"],

    // works apart some render issues

    // bad
    // ['"area" = pi dot "radius"^2', '\\text{area} = \\pi \\cdot \\text{radius}^2'],
    // ['cal(A) := { x in RR | x "is natural" }', ''],
    // ['x < y => x gt.eq.not y', ''],
    // ['sum_(k=0)^n k &= 1 + ... + n \ &= (n(n+1)) / 2', ''],
    // ['frac(a^2, 2)', ''],
    // ['vec(1, 2, delim: "[")', ''],
    // ['mat(1, 2; 3, 4)', ''],
    // ['lim_x = op("lim", limits: #true)_x', ''],
    // ['(3x + y) / 7 &= 9 && "given" \ 3x + y &= 63 & "multiply by 7" \ 3x &= 63 - y && "subtract y" \ x &= 21 - y/3 & "divide by 3"', ''],
    // ['sum_(i=0)^n a_i = 2^(1+i)', ''],
    // ['1/2 < (x+1)/2', ''],
    // ['((x+1)) / 2 = frac(a, b)', ''],
    // ['tan x = (sin x)/(cos x)', ''],
    // ['op("custom", limits: #true)_(n->oo) n', ''],
    // ['bb(b)', ''],
    // ['bb(N) = NN', ''],
    // ['f: NN -> RR', ''],
    // ['vec(a, b, c) dot vec(1, 2, 3) = a + 2b + 3c', ''],
    // ['attach(Pi, t: alpha, b: beta, tl: 1, tr: 2+3, bl: 4+5, br: 6)', ''],
    // ['lr(]sum_(x=1)^n] x, size: #50%)', ''],
    // ['mat(1, 2, ..., 10; 2, 2, ..., 10; dots.v, dots.v, dots.down, dots.v; 10, 10, ..., 10)', ''],
    // ['upright(A) != A', ''],
];

export async function runTests() {
    let errors = {};
    let core = await loadTypst();

    for (let expr of exprs) {
        let latexKatexTree = katexParseTree(expr[0]);
        let typstKatexTree = parseTree(expr[1], new Settings({}));

        deleteFields(latexKatexTree, ['loc']);
        deleteFields(typstKatexTree, ['loc']);
        deleteIfFieldEquals(latexKatexTree, undefined);
        deleteIfFieldEquals(typstKatexTree, undefined);

        let fmt = `Expected coreTree to deep equal tree. \nTypstSyntaxTree: ${core.typst_tree_str(expr[0])}\nCoreTree: ${JSON.stringify(latexKatexTree, null, 2)} \nTree: ${JSON.stringify(typstKatexTree, null, 2)}`
        try {
            chai.expect(latexKatexTree).to.deep.equal(typstKatexTree, fmt)
        } catch (e) {
            errors[expr[0]] = {
                latex: expr[1],
                katex_tree: {
                    "latex": latexKatexTree,
                    "typst": typstKatexTree,
                    "diff": diff(latexKatexTree, typstKatexTree)
                },
                message: e,
            }
        }
    }

    return errors;
}
