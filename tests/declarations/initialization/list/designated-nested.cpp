// tidal-rules: LIST-14
// tidal-expect: reject
// tidal-error-clang: nested designators are a C99 extension
// tidal-error-edg: chained designators are nonstandard
struct P { int x; };
struct S { P p; };
S s{.p.x = 1};
