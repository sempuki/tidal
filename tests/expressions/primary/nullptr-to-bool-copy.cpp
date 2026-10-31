// tidal-rules: PRIM-24
// tidal-expect: reject
// tidal-error-clang: type .bool. with an rvalue of type .std::nullptr_t.
// tidal-error-gcc: requires direct-initialization
// tidal-error-edg: nullptr_t. cannot be used to initialize an entity of type .bool
bool b = nullptr;
