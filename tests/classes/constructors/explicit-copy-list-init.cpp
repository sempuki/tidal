// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: constructor is explicit in copy-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: cannot use a constructor marked .explicit
struct S { explicit S(int) {} };
S s = {1};
