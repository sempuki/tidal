// tidal-rules: INIT-7
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .bool. with an rvalue of type .std::nullptr_t
// tidal-error-gcc: requires direct-initialization
// tidal-error-edg: nullptr_t. cannot be used to initialize an entity of type .bool
// tidal-note: Gap review 2026-10-31.
bool b = nullptr;
