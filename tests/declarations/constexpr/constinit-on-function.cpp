// tidal-rules: CINIT-2
// tidal-expect: reject
// tidal-error-clang: constinit can only be used in variable declarations
// tidal-error-gcc: constinit. on function return type is not allowed
// tidal-error-edg: constinit. is not valid here
constinit int f();
