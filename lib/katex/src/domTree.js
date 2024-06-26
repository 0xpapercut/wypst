//      
/**
 * These objects store the data about the DOM nodes we create, as well as some
 * extra data. They can then be transformed into real DOM nodes with the
 * `toNode` function or HTML markup using `toMarkup`. They are useful for both
 * storing extra properties on the nodes, as well as providing a way to easily
 * work with the DOM.
 *
 * Similar functions for working with MathML nodes exist in mathMLTree.js.
 *
 * TODO: refactor `span` and `anchor` into common superclass when
 * target environments support class inheritance
 */
import {scriptFromCodepoint} from "./unicodeScripts";
import utils from "./utils";
import {path} from "./svgGeometry";
                                     
import {DocumentFragment} from "./tree";
import {makeEm} from "./units";

                                        


/**
 * Create an HTML className based on a list of classes. In addition to joining
 * with spaces, we also remove empty classes.
 */
export const createClass = function(classes          )         {
    return classes.filter(cls => cls).join(" ");
};

const initNode = function(
    classes           ,
    options          ,
    style           ,
) {
    this.classes = classes || [];
    this.attributes = {};
    this.height = 0;
    this.depth = 0;
    this.maxFontSize = 0;
    this.style = style || {};
    if (options) {
        if (options.style.isTight()) {
            this.classes.push("mtight");
        }
        const color = options.getColor();
        if (color) {
            this.style.color = color;
        }
    }
};

/**
 * Convert into an HTML node
 */
const toNode = function(tagName        )              {
    const node = document.createElement(tagName);

    // Apply the class
    node.className = createClass(this.classes);

    // Apply inline styles
    for (const style in this.style) {
        if (this.style.hasOwnProperty(style)) {
            // $FlowFixMe Flow doesn't seem to understand span.style's type.
            node.style[style] = this.style[style];
        }
    }

    // Apply attributes
    for (const attr in this.attributes) {
        if (this.attributes.hasOwnProperty(attr)) {
            node.setAttribute(attr, this.attributes[attr]);
        }
    }

    // Append the children, also as HTML nodes
    for (let i = 0; i < this.children.length; i++) {
        node.appendChild(this.children[i].toNode());
    }

    return node;
};

/**
 * Convert into an HTML markup string
 */
const toMarkup = function(tagName        )         {
    let markup = `<${tagName}`;

    // Add the class
    if (this.classes.length) {
        markup += ` class="${utils.escape(createClass(this.classes))}"`;
    }

    let styles = "";

    // Add the styles, after hyphenation
    for (const style in this.style) {
        if (this.style.hasOwnProperty(style)) {
            styles += `${utils.hyphenate(style)}:${this.style[style]};`;
        }
    }

    if (styles) {
        markup += ` style="${utils.escape(styles)}"`;
    }

    // Add the attributes
    for (const attr in this.attributes) {
        if (this.attributes.hasOwnProperty(attr)) {
            markup += ` ${attr}="${utils.escape(this.attributes[attr])}"`;
        }
    }

    markup += ">";

    // Add the markup of the children, also as markup
    for (let i = 0; i < this.children.length; i++) {
        markup += this.children[i].toMarkup();
    }

    markup += `</${tagName}>`;

    return markup;
};

// Making the type below exact with all optional fields doesn't work due to
// - https://github.com/facebook/flow/issues/4582
// - https://github.com/facebook/flow/issues/5688
// However, since *all* fields are optional, $Shape<> works as suggested in 5688
// above.
// This type does not include all CSS properties. Additional properties should
// be added as needed.
                               
                            
                              
                        
                             
                             
                           
                        
                        
                   
                  
                   
                 
                   
                       
                        
                      
                     
                        
                     
                       
                
                  
                          
        

                                                  
                      
                   
                  
                        
                    

                                         
 

// Span wrapping other DOM nodes.
                                        
// Span wrapping an SVG node.
                                    

                                               
                                                             


/**
 * This node represents a span node, with a className, a list of children, and
 * an inline style. It also contains information about its height, depth, and
 * maxFontSize.
 *
 * Represents two types with different uses: SvgSpan to wrap an SVG and DomSpan
 * otherwise. This typesafety is important when HTML builders access a span's
 * children.
 */
export class Span                                                {
    children             ;
    attributes                    ;
    classes          ;
    height        ;
    depth        ;
    width         ;
    maxFontSize        ;
    style          ;

    constructor(
        classes           ,
        children              ,
        options          ,
        style           ,
    ) {
        initNode.call(this, classes, options, style);
        this.children = children || [];
    }

    /**
     * Sets an arbitrary attribute on the span. Warning: use this wisely. Not
     * all browsers support attributes the same, and having too many custom
     * attributes is probably bad.
     */
    setAttribute(attribute        , value        ) {
        this.attributes[attribute] = value;
    }

    hasClass(className        )          {
        return utils.contains(this.classes, className);
    }

    toNode()              {
        return toNode.call(this, "span");
    }

    toMarkup()         {
        return toMarkup.call(this, "span");
    }
}

/**
 * This node represents an anchor (<a>) element with a hyperlink.  See `span`
 * for further details.
 */
export class Anchor                        {
    children               ;
    attributes                    ;
    classes          ;
    height        ;
    depth        ;
    maxFontSize        ;
    style          ;

    constructor(
        href        ,
        classes          ,
        children               ,
        options         ,
    ) {
        initNode.call(this, classes, options);
        this.children = children || [];
        this.setAttribute('href', href);
    }

    setAttribute(attribute        , value        ) {
        this.attributes[attribute] = value;
    }

    hasClass(className        )          {
        return utils.contains(this.classes, className);
    }

    toNode()              {
        return toNode.call(this, "a");
    }

    toMarkup()         {
        return toMarkup.call(this, "a");
    }
}

/**
 * This node represents an image embed (<img>) element.
 */
export class Img                        {
    src        ;
    alt        ;
    classes          ;
    height        ;
    depth        ;
    maxFontSize        ;
    style          ;

    constructor(
        src        ,
        alt        ,
        style          ,
    ) {
        this.alt = alt;
        this.src = src;
        this.classes = ["mord"];
        this.style = style;
    }

    hasClass(className        )          {
        return utils.contains(this.classes, className);
    }

    toNode()       {
        const node = document.createElement("img");
        node.src = this.src;
        node.alt = this.alt;
        node.className = "mord";

        // Apply inline styles
        for (const style in this.style) {
            if (this.style.hasOwnProperty(style)) {
                // $FlowFixMe
                node.style[style] = this.style[style];
            }
        }

        return node;
    }

    toMarkup()         {
        let markup = `<img src="${utils.escape(this.src)}"` +
          ` alt="${utils.escape(this.alt)}"`;

        // Add the styles, after hyphenation
        let styles = "";
        for (const style in this.style) {
            if (this.style.hasOwnProperty(style)) {
                styles += `${utils.hyphenate(style)}:${this.style[style]};`;
            }
        }
        if (styles) {
            markup += ` style="${utils.escape(styles)}"`;
        }

        markup += "'/>";
        return markup;
    }
}

const iCombinations = {
    'î': '\u0131\u0302',
    'ï': '\u0131\u0308',
    'í': '\u0131\u0301',
    // 'ī': '\u0131\u0304', // enable when we add Extended Latin
    'ì': '\u0131\u0300',
};

/**
 * A symbol node contains information about a single symbol. It either renders
 * to a single text node, or a span with a single text node in it, depending on
 * whether it has CSS classes, styles, or needs italic correction.
 */
export class SymbolNode                        {
    text        ;
    height        ;
    depth        ;
    italic        ;
    skew        ;
    width        ;
    maxFontSize        ;
    classes          ;
    style          ;

    constructor(
        text        ,
        height         ,
        depth         ,
        italic         ,
        skew         ,
        width         ,
        classes           ,
        style           ,
    ) {
        this.text = text;
        this.height = height || 0;
        this.depth = depth || 0;
        this.italic = italic || 0;
        this.skew = skew || 0;
        this.width = width || 0;
        this.classes = classes || [];
        this.style = style || {};
        this.maxFontSize = 0;

        // Mark text from non-Latin scripts with specific classes so that we
        // can specify which fonts to use.  This allows us to render these
        // characters with a serif font in situations where the browser would
        // either default to a sans serif or render a placeholder character.
        // We use CSS class names like cjk_fallback, hangul_fallback and
        // brahmic_fallback. See ./unicodeScripts.js for the set of possible
        // script names
        const script = scriptFromCodepoint(this.text.charCodeAt(0));
        if (script) {
            this.classes.push(script + "_fallback");
        }

        if (/[îïíì]/.test(this.text)) {    // add ī when we add Extended Latin
            this.text = iCombinations[this.text];
        }
    }

    hasClass(className        )          {
        return utils.contains(this.classes, className);
    }

    /**
     * Creates a text node or span from a symbol node. Note that a span is only
     * created if it is needed.
     */
    toNode()       {
        const node = document.createTextNode(this.text);
        let span = null;

        if (this.italic > 0) {
            span = document.createElement("span");
            span.style.marginRight = makeEm(this.italic);
        }

        if (this.classes.length > 0) {
            span = span || document.createElement("span");
            span.className = createClass(this.classes);
        }

        for (const style in this.style) {
            if (this.style.hasOwnProperty(style)) {
                span = span || document.createElement("span");
                // $FlowFixMe Flow doesn't seem to understand span.style's type.
                span.style[style] = this.style[style];
            }
        }

        if (span) {
            span.appendChild(node);
            return span;
        } else {
            return node;
        }
    }

    /**
     * Creates markup for a symbol node.
     */
    toMarkup()         {
        // TODO(alpert): More duplication than I'd like from
        // span.prototype.toMarkup and symbolNode.prototype.toNode...
        let needsSpan = false;

        let markup = "<span";

        if (this.classes.length) {
            needsSpan = true;
            markup += " class=\"";
            markup += utils.escape(createClass(this.classes));
            markup += "\"";
        }

        let styles = "";

        if (this.italic > 0) {
            styles += "margin-right:" + this.italic + "em;";
        }
        for (const style in this.style) {
            if (this.style.hasOwnProperty(style)) {
                styles += utils.hyphenate(style) + ":" + this.style[style] + ";";
            }
        }

        if (styles) {
            needsSpan = true;
            markup += " style=\"" + utils.escape(styles) + "\"";
        }

        const escaped = utils.escape(this.text);
        if (needsSpan) {
            markup += ">";
            markup += escaped;
            markup += "</span>";
            return markup;
        } else {
            return escaped;
        }
    }
}

/**
 * SVG nodes are used to render stretchy wide elements.
 */
export class SvgNode                        {
    children                ;
    attributes                    ;

    constructor(children                 , attributes                     ) {
        this.children = children || [];
        this.attributes = attributes || {};
    }

    toNode()       {
        const svgNS = "http://www.w3.org/2000/svg";
        const node = document.createElementNS(svgNS, "svg");

        // Apply attributes
        for (const attr in this.attributes) {
            if (Object.prototype.hasOwnProperty.call(this.attributes, attr)) {
                node.setAttribute(attr, this.attributes[attr]);
            }
        }

        for (let i = 0; i < this.children.length; i++) {
            node.appendChild(this.children[i].toNode());
        }
        return node;
    }

    toMarkup()         {
        let markup = `<svg xmlns="http://www.w3.org/2000/svg"`;

        // Apply attributes
        for (const attr in this.attributes) {
            if (Object.prototype.hasOwnProperty.call(this.attributes, attr)) {
                markup += ` ${attr}="${utils.escape(this.attributes[attr])}"`;
            }
        }

        markup += ">";

        for (let i = 0; i < this.children.length; i++) {
            markup += this.children[i].toMarkup();
        }

        markup += "</svg>";

        return markup;

    }
}

export class PathNode                        {
    pathName        ;
    alternate         ;

    constructor(pathName        , alternate         ) {
        this.pathName = pathName;
        this.alternate = alternate;  // Used only for \sqrt, \phase, & tall delims
    }

    toNode()       {
        const svgNS = "http://www.w3.org/2000/svg";
        const node = document.createElementNS(svgNS, "path");

        if (this.alternate) {
            node.setAttribute("d", this.alternate);
        } else {
            node.setAttribute("d", path[this.pathName]);
        }

        return node;
    }

    toMarkup()         {
        if (this.alternate) {
            return `<path d="${utils.escape(this.alternate)}"/>`;
        } else {
            return `<path d="${utils.escape(path[this.pathName])}"/>`;
        }
    }
}

export class LineNode                        {
    attributes                    ;

    constructor(attributes                     ) {
        this.attributes = attributes || {};
    }

    toNode()       {
        const svgNS = "http://www.w3.org/2000/svg";
        const node = document.createElementNS(svgNS, "line");

        // Apply attributes
        for (const attr in this.attributes) {
            if (Object.prototype.hasOwnProperty.call(this.attributes, attr)) {
                node.setAttribute(attr, this.attributes[attr]);
            }
        }

        return node;
    }

    toMarkup()         {
        let markup = "<line";

        for (const attr in this.attributes) {
            if (Object.prototype.hasOwnProperty.call(this.attributes, attr)) {
                markup += ` ${attr}="${utils.escape(this.attributes[attr])}"`;
            }
        }

        markup += "/>";

        return markup;
    }
}

export function assertSymbolDomNode(
    group             ,
)             {
    if (group instanceof SymbolNode) {
        return group;
    } else {
        throw new Error(`Expected symbolNode but got ${String(group)}.`);
    }
}

export function assertSpan(
    group             ,
)                    {
    if (group instanceof Span) {
        return group;
    } else {
        throw new Error(`Expected span<HtmlDomNode> but got ${String(group)}.`);
    }
}
