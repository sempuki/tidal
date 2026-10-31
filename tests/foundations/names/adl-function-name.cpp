// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct S { }; int probe(void (*)(S)) { return 1; } }
void use(n::S) { }
int read() { return probe(use); }
