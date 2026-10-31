// tidal-rules: PFX-17
// tidal-expect: reject
// tidal-error-clang: does not provide a call operator
// tidal-error-gcc: no match for call to
// tidal-error-edg: without appropriate operator\(\)
struct Box {};
void run(Box b) { b(); }
