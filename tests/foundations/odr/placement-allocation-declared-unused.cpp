// tidal-rules: ODR-8
// tidal-expect: run
// tidal-note: Roadmap round 7 programmer doubt, round 8 writer probe placement-new-declared-only.cpp. A placement allocation function with external linkage is declared and never defined. The program does not use it, so it needs no definition.
using size_t = decltype(sizeof 0);
struct Arena { alignas(16) unsigned char buf[64]; };
void* operator new(size_t, Arena&);
int main() { return 0; }
