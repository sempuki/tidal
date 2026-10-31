// tidal-rules: NEW-16
// tidal-expect: reject
// tidal-error-clang: new expression for type .auto. requires a constructor argument
// tidal-error-gcc: type .auto. requires exactly one element
// tidal-error-edg: cannot deduce .auto. type
void f() { auto p = new auto; }
