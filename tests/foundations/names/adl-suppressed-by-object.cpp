// tidal-rules: NAM-32
// tidal-expect: reject
// tidal-error-clang: no matching function for call to object of type .caller.
// tidal-error-gcc: no match for call to ..caller. .n::S..
// tidal-error-edg: function .caller::operator... cannot be called with the given argument list
namespace n { struct S { }; int probe(S) { return 1; } }
struct caller { int operator()(int) const { return 2; } };
caller probe;
int read() { return probe(n::S{}); }
