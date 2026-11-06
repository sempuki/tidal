// tidal-rules: ODR-8 LSUP-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference
// tidal-error-gcc: undefined reference
// tidal-note: Roadmap round 7 programmer doubt, round 8 writer probe placement-new-declared-only.cpp. A new expression uses a placement allocation function that the program declares and never defines. ODR-8 needs no diagnostic, and the link fails on every compiler that links. The program must define the function (LSUP-7).
using size_t = decltype(sizeof 0);
struct Arena { alignas(16) unsigned char buf[64]; };
void* operator new(size_t, Arena&);
struct S { int x; };
int main() {
  Arena a;
  S* p = new (a) S{1};
  return p->x - 1;
}
