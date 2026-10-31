// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: constructor is explicit in copy-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: no default constructor exists for class .S
struct S { explicit S() {} };
S make() { return {}; }
