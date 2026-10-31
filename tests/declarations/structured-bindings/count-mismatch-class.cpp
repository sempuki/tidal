// tidal-rules: SB-2
// tidal-expect: reject
// tidal-error-clang: but only 1 name was provided
// tidal-error-gcc: only 1 name provided for structured binding
// tidal-error-edg: more elements than there are binding names
struct S { int a; int b; };
S s;
auto [x] = s;
