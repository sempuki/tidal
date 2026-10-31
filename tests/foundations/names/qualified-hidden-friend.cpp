// tidal-rules: NAM-14 ACC-7
// tidal-expect: reject
// tidal-error-clang: no member named .probe. in namespace .n.
// tidal-error-gcc: probe. is not a member of .n
// tidal-error-edg: namespace .n. has no member .probe.
namespace n { struct S { friend int probe(S) { return 1; } }; }
int read() { return n::probe(n::S{}); }
