// tidal-rules: NAM-32
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .n::S. to .int.
// tidal-error-gcc: cannot convert .n::S. to .int.
// tidal-error-edg: no suitable conversion function from .n::S. to .int. exists
namespace n { struct S { }; int probe(S) { return 1; } }
struct user { int probe(int) { return 2; } int read() { return probe(n::S{}); } };
