// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: post-increment operator must have type .int
// tidal-error-gcc: must have .int. as its argument
// tidal-error-edg: postfix .operator\+\+. must be of type .int
struct C { C operator++(long); };
