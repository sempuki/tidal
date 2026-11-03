// tidal-rules: LSP-5
// tidal-expect: reject
// tidal-error-clang: unknown linkage language
// tidal-error-gcc: language string .*not recognized
// tidal-error-edg: unknown external linkage specification
// tidal-note: The block form with an unknown language.
extern "Pascal" {
int g();
}
