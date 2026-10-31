// tidal-rules: CAST-4
// tidal-expect: reject
// tidal-error-clang: const_cast to .* which is not a reference, pointer-to-object
// tidal-error-gcc: pointer or reference to a function type
// tidal-error-edg: type in a const_cast must be a pointer
// tidal-note: Gap review 2026-10-31.
void g();
void (*p)() = const_cast<void (*)()>(&g);
