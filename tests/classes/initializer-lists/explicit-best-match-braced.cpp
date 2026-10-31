// tidal-rules: ILC-8
// tidal-expect: reject
// tidal-error-clang: constructor is explicit in copy-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: cannot use a constructor marked .explicit
// tidal-note: Gap review 2026-10-31.
struct S { explicit S(int) {} S(long) {} };
S s = {1};
