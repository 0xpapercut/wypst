//      
import {defineFunctionBuilders} from "../defineFunction";
import buildCommon from "../buildCommon";
import mathMLTree from "../mathMLTree";
import ParseError from "../ParseError";

// A map of CSS-based spacing functions to their CSS class.
const cssSpace                     = {
    "\\nobreak": "nobreak",
    "\\allowbreak": "allowbreak",
};

// A lookup table to determine whether a spacing function/symbol should be
// treated like a regular space character.  If a symbol or command is a key
// in this table, then it should be a regular space character.  Furthermore,
// the associated value may have a `className` specifying an extra CSS class
// to add to the created `span`.
const regularSpace                                     = {
    " ": {},
    "\\ ": {},
    "~": {
        className: "nobreak",
    },
    "\\space": {},
    "\\nobreakspace": {
        className: "nobreak",
    },
};

// ParseNode<"spacing"> created in Parser.js from the "spacing" symbol Groups in
// src/symbols.js.
defineFunctionBuilders({
    type: "spacing",
    htmlBuilder(group, options) {
        if (regularSpace.hasOwnProperty(group.text)) {
            const className = regularSpace[group.text].className || "";
            // Spaces are generated by adding an actual space. Each of these
            // things has an entry in the symbols table, so these will be turned
            // into appropriate outputs.
            if (group.mode === "text") {
                const ord = buildCommon.makeOrd(group, options, "textord");
                ord.classes.push(className);
                return ord;
            } else {
                return buildCommon.makeSpan(["mspace", className],
                    [buildCommon.mathsym(group.text, group.mode, options)],
                    options);
            }
        } else if (cssSpace.hasOwnProperty(group.text)) {
            // Spaces based on just a CSS class.
            return buildCommon.makeSpan(
                ["mspace", cssSpace[group.text]],
                [], options);
        } else {
            throw new ParseError(`Unknown type of space "${group.text}"`);
        }
    },
    mathmlBuilder(group, options) {
        let node;

        if (regularSpace.hasOwnProperty(group.text)) {
            node = new mathMLTree.MathNode(
                "mtext", [new mathMLTree.TextNode("\u00a0")]);
        } else if (cssSpace.hasOwnProperty(group.text)) {
            // CSS-based MathML spaces (\nobreak, \allowbreak) are ignored
            return new mathMLTree.MathNode("mspace");
        } else {
            throw new ParseError(`Unknown type of space "${group.text}"`);
        }

        return node;
    },
});
