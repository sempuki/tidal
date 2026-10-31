// tidal-rules: NEW-16
// tidal-expect: reject
// tidal-error-clang: contains multiple constructor arguments
// tidal-error-gcc: type .auto. requires exactly one element
// tidal-error-edg: expected a .\).
void f() { auto p = new auto(1, 2); }
