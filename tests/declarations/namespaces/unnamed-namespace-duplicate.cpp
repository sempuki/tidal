// tidal-rules: NS-13
// tidal-expect: reject
// tidal-error-clang: reference to .z. is ambiguous
// tidal-error-gcc: reference to .z. is ambiguous
// tidal-error-edg: z. is ambiguous
// tidal-note: Gap review 2026-10-30.
namespace { int z; }
int z;
int w = z;
