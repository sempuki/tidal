// tidal-rules: CAST-2
// tidal-expect: reject
// tidal-error-clang: static_cast from .void \(\*\)\(\). to .void \*. is not allowed
// tidal-error-gcc: invalid .static_cast. from type .void \(\*\)\(\).
// tidal-error-edg: invalid type conversion
// tidal-note: Gap review 2026-10-31.
void g();
void* p = static_cast<void*>(&g);
