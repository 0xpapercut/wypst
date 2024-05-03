//      
                                            

/**
 * Lexing or parsing positional information for error reporting.
 * This object is immutable.
 */
export default class SourceLocation {
    // The + prefix indicates that these fields aren't writeable
     lexer                ; // Lexer holding the input string.
     start        ;         // Start offset, zero-based inclusive.
     end        ;           // End offset, zero-based exclusive.

    constructor(lexer                , start        , end        ) {
        this.lexer = lexer;
        this.start = start;
        this.end = end;
    }

    /**
     * Merges two `SourceLocation`s from location providers, given they are
     * provided in order of appearance.
     * - Returns the first one's location if only the first is provided.
     * - Returns a merged range of the first and the last if both are provided
     *   and their lexers match.
     * - Otherwise, returns null.
     */
    static range(
        first                         ,
        second                         ,
    )                  {
        if (!second) {
            return first && first.loc;
        } else if (!first || !first.loc || !second.loc ||
                   first.loc.lexer !== second.loc.lexer) {
            return null;
        } else {
            return new SourceLocation(
                    first.loc.lexer, first.loc.start, second.loc.end);
        }
    }
}
