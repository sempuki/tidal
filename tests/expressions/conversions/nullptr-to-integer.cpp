// tidal-rules: CONV-18
// tidal-expect: reject
// tidal-error-clang: type .std::nullptr_t
// tidal-error-gcc: cannot convert .std::nullptr_t. to .int
// tidal-error-edg: type "std::nullptr_t" cannot be used to initialize
int n = nullptr;
