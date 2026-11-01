// tidal-rules: FN-15 FN-14
// tidal-expect: reject
// tidal-error-gcc: redefinition of
// tidal-error-clang: redefinition of
// tidal-error-edg: already been defined
// tidal-note: A top-level const on a parameter is not part of the function type (DCL-8), so the two definitions define one function twice.
int scale(int v) { return v; }
int scale(const int v) { return v; }
