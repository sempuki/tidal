// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-error-clang: anonymous union cannot contain a private data member
// tidal-error-gcc: anonymous union may only have public non-static data members
// tidal-error-edg: nonpublic member is not allowed
struct S { union { private: int i; }; };
