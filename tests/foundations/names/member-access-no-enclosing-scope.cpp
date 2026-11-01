// tidal-rules: NAM-37 PFX-11
// tidal-expect: reject
// tidal-error-gcc: has no member named .outside.
// tidal-error-clang: no member named .outside. in
// tidal-error-edg: has no member .outside.
// tidal-note: Member lookup searches only the class and its bases. The namespace variable outside is not found.
int outside = 1;
struct Meter { int v; };
int main() { Meter m{0}; return m.outside; }
