// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: cannot catch exceptions by rvalue reference
// tidal-error-gcc: catch. parameter to be of rvalue reference type
// tidal-error-edg: rvalue reference is not allowed as a catch type
void g() { try { } catch (int&&) { } }
