// tidal-rules: CE-11
// tidal-expect: reject
// tidal-error-clang: must have integral or unscoped enumeration type
// tidal-error-gcc: must have integral or unscoped enumeration type
// tidal-error-edg: has type .double. instead of the required integral or enum type
enum Ratio { half = 0.5 };
