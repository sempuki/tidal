// tidal-rules: ILC-7
// tidal-expect: reject
// tidal-error-clang: non-aggregate type .* designated initializer
// tidal-error-gcc: designated initializers cannot be used with a non-aggregate
// tidal-error-edg: designator cannot be used with a non-aggregate
struct S { int x; S(int v) : x(v) {} };
S s{.x = 1};
