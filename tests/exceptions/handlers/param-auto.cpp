// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: auto. not allowed in exception declaration
// tidal-error-gcc: auto. parameter not permitted
// tidal-error-edg: auto. is not allowed here
void g() { try { } catch (auto e) { } }
