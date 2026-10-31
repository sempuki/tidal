// tidal-rules: PRIM-24
// tidal-expect: reject
// tidal-error-clang: type .int. with an rvalue of type .std::nullptr_t.
// tidal-error-gcc: cannot convert .std::nullptr_t. to .int.
// tidal-error-edg: nullptr_t. cannot be used to initialize an entity of type .int
int x = nullptr;
