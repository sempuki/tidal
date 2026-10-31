// tidal-rules: LIST-14
// tidal-expect: reject
// tidal-error-clang: field designators to be specified in declaration order
// tidal-error-gcc: designator order for field .* does not match declaration order
// tidal-error-edg: out-of-order initializers are nonstandard
struct S { int a; int b; };
S s{.b = 2, .a = 1};
