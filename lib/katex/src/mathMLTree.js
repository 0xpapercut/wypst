//      
/**
 * These objects store data about MathML nodes. This is the MathML equivalent
 * of the types in domTree.js. Since MathML handles its own rendering, and
 * since we're mainly using MathML to improve accessibility, we don't manage
 * any of the styling state that the plain DOM nodes do.
 *
 * The `toNode` and `toMarkup` functions work similarly to how they do in
 * domTree.js, creating namespaced DOM nodes and HTML text markup respectively.
 */

import utils from "./utils";
import {DocumentFragment} from "./tree";
import {createClass} from "./domTree";
import {makeEm} from "./units";

                                        

/**
 * MathML node types used in KaTeX. For a complete list of MathML nodes, see
 * https://developer.mozilla.org/en-US/docs/Web/MathML/Element.
 */
                          
                                         
                                             
                                                                     
                                 
                                             
                         
                                                 

                                                  
                     
 

                                                             
export function newDocumentFragment(
    children                             
)                   {
    return new DocumentFragment(children);
}

/**
 * This node represents a general purpose MathML node of any type. The
 * constructor requires the type of node to create (for example, `"mo"` or
 * `"mspace"`, corresponding to `<mo>` and `<mspace>` tags).
 */
export class MathNode                        {
    type              ;
    attributes                    ;
    children                             ;
    classes          ;

    constructor(
        type              ,
        children                              ,
        classes           
    ) {
        this.type = type;
        this.attributes = {};
        this.children = children || [];
        this.classes = classes || [];
    }

    /**
     * Sets an attribute on a MathML node. MathML depends on attributes to convey a
     * semantic content, so this is used heavily.
     */
    setAttribute(name        , value        ) {
        this.attributes[name] = value;
    }

    /**
     * Gets an attribute on a MathML node.
     */
    getAttribute(name        )         {
        return this.attributes[name];
    }

    /**
     * Converts the math node into a MathML-namespaced DOM element.
     */
    toNode()       {
        const node = document.createElementNS(
            "http://www.w3.org/1998/Math/MathML", this.type);

        for (const attr in this.attributes) {
            if (Object.prototype.hasOwnProperty.call(this.attributes, attr)) {
                node.setAttribute(attr, this.attributes[attr]);
            }
        }

        if (this.classes.length > 0) {
            node.className = createClass(this.classes);
        }

        for (let i = 0; i < this.children.length; i++) {
            node.appendChild(this.children[i].toNode());
        }

        return node;
    }

    /**
     * Converts the math node into an HTML markup string.
     */
    toMarkup()         {
        let markup = "<" + this.type;

        // Add the attributes
        for (const attr in this.attributes) {
            if (Object.prototype.hasOwnProperty.call(this.attributes, attr)) {
                markup += " " + attr + "=\"";
                markup += utils.escape(this.attributes[attr]);
                markup += "\"";
            }
        }

        if (this.classes.length > 0) {
            markup += ` class ="${utils.escape(createClass(this.classes))}"`;
        }

        markup += ">";

        for (let i = 0; i < this.children.length; i++) {
            markup += this.children[i].toMarkup();
        }

        markup += "</" + this.type + ">";

        return markup;
    }

    /**
     * Converts the math node into a string, similar to innerText, but escaped.
     */
    toText()         {
        return this.children.map(child => child.toText()).join("");
    }
}

/**
 * This node represents a piece of text.
 */
export class TextNode                        {
    text        ;

    constructor(text        ) {
        this.text = text;
    }

    /**
     * Converts the text node into a DOM text node.
     */
    toNode()       {
        return document.createTextNode(this.text);
    }

    /**
     * Converts the text node into escaped HTML markup
     * (representing the text itself).
     */
    toMarkup()         {
        return utils.escape(this.toText());
    }

    /**
     * Converts the text node into a string
     * (representing the text itself).
     */
    toText()         {
        return this.text;
    }
}

/**
 * This node represents a space, but may render as <mspace.../> or as text,
 * depending on the width.
 */
class SpaceNode                        {
    width        ;
    character         ;

    /**
     * Create a Space node with width given in CSS ems.
     */
    constructor(width        ) {
        this.width = width;
        // See https://www.w3.org/TR/2000/WD-MathML2-20000328/chapter6.html
        // for a table of space-like characters.  We use Unicode
        // representations instead of &LongNames; as it's not clear how to
        // make the latter via document.createTextNode.
        if (width >= 0.05555 && width <= 0.05556) {
            this.character = "\u200a";           // &VeryThinSpace;
        } else if (width >= 0.1666 && width <= 0.1667) {
            this.character = "\u2009";           // &ThinSpace;
        } else if (width >= 0.2222 && width <= 0.2223) {
            this.character = "\u2005";           // &MediumSpace;
        } else if (width >= 0.2777 && width <= 0.2778) {
            this.character = "\u2005\u200a";     // &ThickSpace;
        } else if (width >= -0.05556 && width <= -0.05555) {
            this.character = "\u200a\u2063";     // &NegativeVeryThinSpace;
        } else if (width >= -0.1667 && width <= -0.1666) {
            this.character = "\u2009\u2063";     // &NegativeThinSpace;
        } else if (width >= -0.2223 && width <= -0.2222) {
            this.character = "\u205f\u2063";     // &NegativeMediumSpace;
        } else if (width >= -0.2778 && width <= -0.2777) {
            this.character = "\u2005\u2063";     // &NegativeThickSpace;
        } else {
            this.character = null;
        }
    }

    /**
     * Converts the math node into a MathML-namespaced DOM element.
     */
    toNode()       {
        if (this.character) {
            return document.createTextNode(this.character);
        } else {
            const node = document.createElementNS(
                "http://www.w3.org/1998/Math/MathML", "mspace");
            node.setAttribute("width", makeEm(this.width));
            return node;
        }
    }

    /**
     * Converts the math node into an HTML markup string.
     */
    toMarkup()         {
        if (this.character) {
            return `<mtext>${this.character}</mtext>`;
        } else {
            return `<mspace width="${makeEm(this.width)}"/>`;
        }
    }

    /**
     * Converts the math node into a string, similar to innerText.
     */
    toText()         {
        if (this.character) {
            return this.character;
        } else {
            return " ";
        }
    }
}

export default {
    MathNode,
    TextNode,
    SpaceNode,
    newDocumentFragment,
};
