// tidal-rules: LIST-11
// tidal-expect: reject
// tidal-error-clang: excess elements in scalar initializer
// tidal-error-gcc: requires one element in initializer
// tidal-error-edg: too many initializer values
int x{1, 2};
