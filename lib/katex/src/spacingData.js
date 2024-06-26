//      
/**
 * Describes spaces between different classes of atoms.
 */
                                         

const thinspace              = {
    number: 3,
    unit: "mu",
};
const mediumspace              = {
    number: 4,
    unit: "mu",
};
const thickspace              = {
    number: 5,
    unit: "mu",
};

// Making the type below exact with all optional fields doesn't work due to
// - https://github.com/facebook/flow/issues/4582
// - https://github.com/facebook/flow/issues/5688
// However, since *all* fields are optional, $Shape<> works as suggested in 5688
// above.
                               
                      
                     
                      
                      
                       
                        
                        
                        
        

// Spacing relationships for display and text styles
export const spacings                                = {
    mord: {
        mop: thinspace,
        mbin: mediumspace,
        mrel: thickspace,
        minner: thinspace,
    },
    mop: {
        mord: thinspace,
        mop: thinspace,
        mrel: thickspace,
        minner: thinspace,
    },
    mbin: {
        mord: mediumspace,
        mop: mediumspace,
        mopen: mediumspace,
        minner: mediumspace,
    },
    mrel: {
        mord: thickspace,
        mop: thickspace,
        mopen: thickspace,
        minner: thickspace,
    },
    mopen: {},
    mclose: {
        mop: thinspace,
        mbin: mediumspace,
        mrel: thickspace,
        minner: thinspace,
    },
    mpunct: {
        mord: thinspace,
        mop: thinspace,
        mrel: thickspace,
        mopen: thinspace,
        mclose: thinspace,
        mpunct: thinspace,
        minner: thinspace,
    },
    minner: {
        mord: thinspace,
        mop: thinspace,
        mbin: mediumspace,
        mrel: thickspace,
        mopen: thinspace,
        mpunct: thinspace,
        minner: thinspace,
    },
};

// Spacing relationships for script and scriptscript styles
export const tightSpacings                                = {
    mord: {
        mop: thinspace,
    },
    mop: {
        mord: thinspace,
        mop: thinspace,
    },
    mbin: {},
    mrel: {},
    mopen: {},
    mclose: {
        mop: thinspace,
    },
    mpunct: {},
    minner: {
        mop: thinspace,
    },
};
