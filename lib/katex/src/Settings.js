//      
/* eslint no-console:0 */
/**
 * This is a module for storing settings passed into KaTeX. It correctly handles
 * default settings.
 */

import utils from "./utils";
import ParseError from "./ParseError";
import {Token} from "./Token";

                                              
                                            

                            
                                                                          
                        

                                 
                
                          
                    
                          
       
                           
                                     
                    
                          
       
               
                         
                    
                          
       
                     
                               
                      
       
                  
                            
                   
       
                     
                               
                      
       
                    
                              
                                       
       
  
                                                         
                                                                   

                                               

                                     
                                                                               
               
                               
           
                                        
           
                            
           
                                                                                  
                                                                                  
                                                                                  
                                                               
           
                      
           
                           
           
                             
           
                                              
           
                                 
           
                                                                                 
                                                                                  
                             
           
                             
           
                                         
           
                         
           
                                                                                 
                               
           
                                
           
                                                                               
                            
           
                                         
      
  

// TODO: automatically generate documentation
// TODO: check all properties on Settings exist
// TODO: check the type of a property on Settings matches
export const SETTINGS_SCHEMA         = {
    displayMode: {
        type: "boolean",
        description: "Render math in display mode, which puts the math in " +
            "display style (so \\int and \\sum are large, for example), and " +
            "centers the math on the page on its own line.",
        cli: "-d, --display-mode",
    },
    output: {
        type: {enum: ["htmlAndMathml", "html", "mathml"]},
        description: "Determines the markup language of the output.",
        cli: "-F, --format <type>",
    },
    leqno: {
        type: "boolean",
        description: "Render display math in leqno style (left-justified tags).",
    },
    fleqn: {
        type: "boolean",
        description: "Render display math flush left.",
    },
    throwOnError: {
        type: "boolean",
        default: true,
        cli: "-t, --no-throw-on-error",
        cliDescription: "Render errors (in the color given by --error-color) ins" +
            "tead of throwing a ParseError exception when encountering an error.",
    },
    errorColor: {
        type: "string",
        default: "#cc0000",
        cli: "-c, --error-color <color>",
        cliDescription: "A color string given in the format 'rgb' or 'rrggbb' " +
            "(no #). This option determines the color of errors rendered by the " +
            "-t option.",
        cliProcessor: (color) => "#" + color,
    },
    macros: {
        type: "object",
        cli: "-m, --macro <def>",
        cliDescription: "Define custom macro of the form '\\foo:expansion' (use " +
            "multiple -m arguments for multiple macros).",
        cliDefault: [],
        cliProcessor: (def, defs) => {
            defs.push(def);
            return defs;
        },
    },
    minRuleThickness: {
        type: "number",
        description: "Specifies a minimum thickness, in ems, for fraction lines," +
            " `\\sqrt` top lines, `{array}` vertical lines, `\\hline`, " +
            "`\\hdashline`, `\\underline`, `\\overline`, and the borders of " +
            "`\\fbox`, `\\boxed`, and `\\fcolorbox`.",
        processor: (t) => Math.max(0, t),
        cli: "--min-rule-thickness <size>",
        cliProcessor: parseFloat,
    },
    colorIsTextColor: {
        type: "boolean",
        description: "Makes \\color behave like LaTeX's 2-argument \\textcolor, " +
            "instead of LaTeX's one-argument \\color mode change.",
        cli: "-b, --color-is-text-color",
    },
    strict: {
        type: [{enum: ["warn", "ignore", "error"]}, "boolean", "function"],
        description: "Turn on strict / LaTeX faithfulness mode, which throws an " +
            "error if the input uses features that are not supported by LaTeX.",
        cli: "-S, --strict",
        cliDefault: false,
    },
    trust: {
        type: ["boolean", "function"],
        description: "Trust the input, enabling all HTML features such as \\url.",
        cli: "-T, --trust",
    },
    maxSize: {
        type: "number",
        default: Infinity,
        description: "If non-zero, all user-specified sizes, e.g. in " +
            "\\rule{500em}{500em}, will be capped to maxSize ems. Otherwise, " +
            "elements and spaces can be arbitrarily large",
        processor: (s) => Math.max(0, s),
        cli: "-s, --max-size <n>",
        cliProcessor: parseInt,
    },
    maxExpand: {
        type: "number",
        default: 1000,
        description: "Limit the number of macro expansions to the specified " +
            "number, to prevent e.g. infinite macro loops. If set to Infinity, " +
            "the macro expander will try to fully expand as in LaTeX.",
        processor: (n) => Math.max(0, n),
        cli: "-e, --max-expand <n>",
        cliProcessor: (n) => (n === "Infinity" ? Infinity : parseInt(n)),
    },
    globalGroup: {
        type: "boolean",
        cli: false,
    },
};

function getDefaultValue(schema)      {
    if (schema.default) {
        return schema.default;
    }
    const type = schema.type;
    const defaultType = Array.isArray(type) ? type[0] : type;
    if (typeof defaultType !== 'string') {
        return defaultType.enum[0];
    }
    switch (defaultType) {
        case 'boolean':
            return false;
        case 'string':
            return '';
        case 'number':
            return 0;
        case 'object':
            return {};
    }
}

/**
 * The main Settings object
 *
 * The current options stored are:
 *  - displayMode: Whether the expression should be typeset as inline math
 *                 (false, the default), meaning that the math starts in
 *                 \textstyle and is placed in an inline-block); or as display
 *                 math (true), meaning that the math starts in \displaystyle
 *                 and is placed in a block with vertical margin.
 */
export default class Settings {
    displayMode         ;
    output                                     ;
    leqno         ;
    fleqn         ;
    throwOnError         ;
    errorColor        ;
    macros          ;
    minRuleThickness        ;
    colorIsTextColor         ;
    strict                                                        ;
    trust                         ;
    maxSize        ;
    maxExpand        ;
    globalGroup         ;

    constructor(options                 ) {
        // allow null options
        options = options || {};
        for (const prop in SETTINGS_SCHEMA) {
            if (SETTINGS_SCHEMA.hasOwnProperty(prop)) {
                // $FlowFixMe
                const schema = SETTINGS_SCHEMA[prop];
                // TODO: validate options
                // $FlowFixMe
                this[prop] = options[prop] !== undefined ? (schema.processor
                        ? schema.processor(options[prop]) : options[prop])
                    : getDefaultValue(schema);
            }
        }
    }

    /**
     * Report nonstrict (non-LaTeX-compatible) input.
     * Can safely not be called if `this.strict` is false in JavaScript.
     */
    reportNonstrict(errorCode        , errorMsg        ,
                    token                       ) {
        let strict = this.strict;
        if (typeof strict === "function") {
            // Allow return value of strict function to be boolean or string
            // (or null/undefined, meaning no further processing).
            strict = strict(errorCode, errorMsg, token);
        }
        if (!strict || strict === "ignore") {
            return;
        } else if (strict === true || strict === "error") {
            throw new ParseError(
                "LaTeX-incompatible input and strict mode is set to 'error': " +
                `${errorMsg} [${errorCode}]`, token);
        } else if (strict === "warn") {
            typeof console !== "undefined" && console.warn(
                "LaTeX-incompatible input and strict mode is set to 'warn': " +
                `${errorMsg} [${errorCode}]`);
        } else {  // won't happen in type-safe code
            typeof console !== "undefined" && console.warn(
                "LaTeX-incompatible input and strict mode is set to " +
                `unrecognized '${strict}': ${errorMsg} [${errorCode}]`);
        }
    }

    /**
     * Check whether to apply strict (LaTeX-adhering) behavior for unusual
     * input (like `\\`).  Unlike `nonstrict`, will not throw an error;
     * instead, "error" translates to a return value of `true`, while "ignore"
     * translates to a return value of `false`.  May still print a warning:
     * "warn" prints a warning and returns `false`.
     * This is for the second category of `errorCode`s listed in the README.
     */
    useStrictBehavior(errorCode        , errorMsg        ,
                      token                       )          {
        let strict = this.strict;
        if (typeof strict === "function") {
            // Allow return value of strict function to be boolean or string
            // (or null/undefined, meaning no further processing).
            // But catch any exceptions thrown by function, treating them
            // like "error".
            try {
                strict = strict(errorCode, errorMsg, token);
            } catch (error) {
                strict = "error";
            }
        }
        if (!strict || strict === "ignore") {
            return false;
        } else if (strict === true || strict === "error") {
            return true;
        } else if (strict === "warn") {
            typeof console !== "undefined" && console.warn(
                "LaTeX-incompatible input and strict mode is set to 'warn': " +
                `${errorMsg} [${errorCode}]`);
            return false;
        } else {  // won't happen in type-safe code
            typeof console !== "undefined" && console.warn(
                "LaTeX-incompatible input and strict mode is set to " +
                `unrecognized '${strict}': ${errorMsg} [${errorCode}]`);
            return false;
        }
    }

    /**
     * Check whether to test potentially dangerous input, and return
     * `true` (trusted) or `false` (untrusted).  The sole argument `context`
     * should be an object with `command` field specifying the relevant LaTeX
     * command (as a string starting with `\`), and any other arguments, etc.
     * If `context` has a `url` field, a `protocol` field will automatically
     * get added by this function (changing the specified object).
     */
    isTrusted(context                 )          {
        if (context.url && !context.protocol) {
            const protocol = utils.protocolFromUrl(context.url);
            if (protocol == null) {
                return false;
            }
            context.protocol = protocol;
        }
        const trust = typeof this.trust === "function"
            ? this.trust(context)
            : this.trust;
        return Boolean(trust);
    }
}
