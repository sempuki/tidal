// tidal-rules: SB-3
// tidal-expect: reject
// tidal-error-clang: because it has an anonymous union member
// tidal-error-gcc: because it has an anonymous union member
// tidal-error-edg: has no components to bind to
struct S { union { int a; }; int b; };
S s;
auto [x, y] = s;
