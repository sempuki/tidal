// tidal-rules: LOG-3
// tidal-expect: reject
// tidal-error-clang: value of type .void. is not contextually convertible to .bool
// tidal-error-gcc: from .void. to .bool.
// tidal-error-edg: must have bool type
bool check(bool ok) { return ok || (throw 1); }
