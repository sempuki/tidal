// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-error-clang: static data member .n. not allowed in anonymous union
// tidal-error-gcc: static data member .* in unnamed class
// tidal-error-edg: static data member declaration is not allowed in this class
struct S { union { static int n; int i; }; };
