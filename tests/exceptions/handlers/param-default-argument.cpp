// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: expected .\).
// tidal-error-gcc: expected .\). before .=. token
// tidal-error-edg: expected a .\).
// tidal-note: The exception_decl production derives no default argument.
void g() { try { } catch (int v = 0) { } }
