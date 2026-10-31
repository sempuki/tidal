// tidal-rules: LIST-5
// tidal-expect: reject
// tidal-error-clang: non-aggregate type .int. with a designated initializer list
// tidal-error-gcc: designator .a. outside aggregate initializer
// tidal-error-edg: designator cannot be used with a non-aggregate type
int x{.a = 1};
