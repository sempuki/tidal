// tidal-rules: TRY-16
// tidal-expect: reject
// tidal-error-clang: expected catch
// tidal-error-gcc: expected .catch.
// tidal-error-edg: try block requires at least one handler
// tidal-note: A function try block with no handler.
void g() try { }
