//      
import {_htmlGroupBuilders, _mathmlGroupBuilders} from "./defineFunction";

                                   
                                              
                                           
                                          
                                                                 

/**
 * The context contains the following properties:
 *  - mode: current parsing mode.
 *  - envName: the name of the environment, one of the listed names.
 *  - parser: the parser object.
 */
                    
               
                    
                   
   

/**
 *  - context: information and references provided by the parser
 *  - args: an array of arguments passed to \begin{name}
 *  - optArgs: an array of optional arguments passed to \begin{name}
 */
                   
                        
                         
                               
                  

/**
 *  - numArgs: (default 0) The number of arguments after the \begin{name} function.
 *  - argTypes: (optional) Just like for a function
 *  - allowedInText: (default false) Whether or not the environment is allowed
 *                   inside text mode (not enforced yet).
 *  - numOptionalArgs: (default 0) Just like for a function
 */
                 
                    
  

/**
 * Final environment spec for use at parse time.
 * This is almost identical to `EnvDefSpec`, except it
 * 1. includes the function handler
 * 2. requires all arguments except argType
 * It is generated by `defineEnvironment()` below.
 */
                                            
                                                                            
                    
                         
                           
                            
                        
   

/**
 * All registered environments.
 * `environments.js` exports this same dictionary again and makes it public.
 * `Parser.js` requires this dictionary via `environments.js`.
 */
export const _environments                         = {};

                                        
                                                  
                   

                                                                 
                         
                         

                                                               
                    

                        

                                                                           
                                                         
                                       

                                                                              
                                                         
                                           
   

export default function defineEnvironment                    ({
    type,
    names,
    props,
    handler,
    htmlBuilder,
    mathmlBuilder,
}                      ) {
    // Set default values of environments.
    const data = {
        type,
        numArgs: props.numArgs || 0,
        allowedInText: false,
        numOptionalArgs: 0,
        handler,
    };
    for (let i = 0; i < names.length; ++i) {
        // TODO: The value type of _environments should be a type union of all
        // possible `EnvSpec<>` possibilities instead of `EnvSpec<*>`, which is
        // an existential type.
        _environments[names[i]] = data;
    }
    if (htmlBuilder) {
        _htmlGroupBuilders[type] = htmlBuilder;
    }
    if (mathmlBuilder) {
        _mathmlGroupBuilders[type] = mathmlBuilder;
    }
}