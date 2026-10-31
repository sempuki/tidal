// tidal-rules: NEW-16
// tidal-expect: reject
// tidal-error-clang: cannot allocate array of .auto.
// tidal-error-gcc: type .auto. requires exactly one element
// tidal-error-edg: auto. type cannot appear in top-level array type
void f() { auto p = new auto[2]{1, 2}; }
