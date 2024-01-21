const fs = require('fs');
const vm = require('vm');
const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;
const generator = require('@babel/generator').default;

const data = fs.readFileSync('scripts/dist/symbols.js', 'utf8');
const ast = parser.parse(data);

global.rustLines = []
global.atoms = ['bin', 'close', 'inner', 'open', 'punct', 'rel']
global.nodeMap = {
    'mathord': 'MathOrd',
    'textord': 'TextOrd',
}

global.toPascalCase = function(str) {
    return str.replace(/(^\w|-\w|_\w)/g, (group) => group.toUpperCase()
                                                          .replace('-', '')
                                                          .replace('_', ''));
}

global.getRustGroup = function(group) {
    if (atoms.includes(group)) {
        return `Group::Atom(AtomGroup::${toPascalCase(group)})`;
    } else {
        if (group === 'mathord')
            group = 'MathOrd';
        else if (group === 'textord')
            group = 'TextOrd';
        return `Group::NonAtom(NonAtomGroup::${toPascalCase(group)})`;
    }
}

global.combinations = new Object();

function newDefineSymbol(mode, font, group, replace, name, ...args) {
    let rustMode = mode === 'math' ? 'Mode::Math' : 'Mode::Text';
    let rustFont = font === 'main' ? 'Font::Main' : 'Font::Ams';
    let rustGroup = getRustGroup(group);
    if (replace === null)
        return
    let rustName = replace === '\\' ? '\\\\' : replace;
    if (replace === '0') {
        console.log('0');
    }
    if (replace === 'A') {
        console.log(mode, font, group);
    }
    let rustLine = `if mode == ${rustMode} && name == '${rustName}' { return Symbol::new(${rustMode}, ${rustFont}, ${rustGroup}, '${rustName}'); }`;
    global.combinations[JSON.stringify([rustMode, rustName])] = rustLine;
    // global.combinations.add(JSON.stringify([rustMode, rustName]));
    // rustLines.push(rustLine);
}

console.log('Im here');

traverse(ast, {
    enter(path) {
        if (path.node.type === 'FunctionDeclaration' && path.node.id.name === 'defineSymbol') {
            path.replaceWith(
                parser.parse(newDefineSymbol.toString()).program.body[0]
            )
        }
        if (path.node.type === 'CallExpression' && path.node.callee.name === 'defineSymbol') {
            path.node.callee.name = 'newDefineSymbol';
        }
    }
})

let { code } = generator(ast);
vm.runInThisContext(code);

const blockStart = '//// --- AUTO GENERATED CODE --- ////'
const blockEnd = '//// --------------------------- ////'

const rustSource = fs.readFileSync('src/core/src/katex/symbol.rs', 'utf8');

let lines = rustSource.split('\n');
const blockStartIndex = lines.findIndex(line => line.includes(blockStart));
const blockEndIndex = lines.findIndex(line => line.includes(blockEnd));

let indentationSpace = lines[blockStartIndex].slice(0, lines[blockStartIndex].indexOf(blockStart));
global.rustLines = Object.values(global.combinations);
let rustLines = global.rustLines.map(s => indentationSpace + s);
lines = lines.slice(0, blockStartIndex + 1).concat(rustLines).concat(lines.slice(blockEndIndex));

fs.writeFileSync('src/core/src/katex/symbol.rs', lines.join('\n'), 'utf8');



// console.log(data.slice(0, data.indexOf(blockStart)) + blockStart + '\n' + global.rustLines.slice(0, 3).join('\n') + '\n' + blockEnd + data.slice(data.indexOf(blockEnd) + blockEnd.length))

// const result = data.replace(start)
// console.log(data)


// fs.readFile('src/core/src/katex/symbol.rs', 'utf8', function(err, data) {
//     if (err) {
//         return console.log(err);
//     }

//     let lines = data.split('\n');
//     const blockStartIndex = lines.findIndex(line => line.includes(blockStart))
//     const blockEndIndex = lines.findIndex(line => line.includes(blockEnd))

//     let indentationSpace = lines[blockStartIndex].slice(0, lines[blockStartIndex].indexOf(blockStart))
//     let rustLines = global.rustLines.map(s => indentationSpace + s).slice(0, 3)
//     lines = lines.slice(0, blockStartIndex + 1).concat(rustLines).concat(lines.slice(blockEndIndex))
//     fs.writeFileSync()


//     // console.log(data.slice(0, data.indexOf(blockStart)) + blockStart + '\n' + global.rustLines.slice(0, 3).join('\n') + '\n' + blockEnd + data.slice(data.indexOf(blockEnd) + blockEnd.length))

//     // const result = data.replace(start)
//     // console.log(data)
// })
