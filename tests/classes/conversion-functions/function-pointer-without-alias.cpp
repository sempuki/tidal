// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: cannot specify any part of a return type in the declaration of a conversion function
int seven();
struct S { operator int (*())(); };
