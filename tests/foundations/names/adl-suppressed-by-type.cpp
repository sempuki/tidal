// tidal-rules: NAM-32
// tidal-expect: reject
// tidal-error-clang: no matching conversion for functional-style cast from .n::S. to .probe.
// tidal-error-gcc: no matching function for call to .probe::probe.n::S..
// tidal-error-edg: no instance of constructor .probe::probe. matches the argument list
namespace n { struct S { }; int probe(S) { return 1; } }
struct probe { probe(int) { } };
auto read() { return probe(n::S{}); }
