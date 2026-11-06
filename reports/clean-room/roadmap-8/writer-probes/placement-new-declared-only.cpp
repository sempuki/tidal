// Writer probe, round 8: a placement allocation function that is declared
// and never defined. USE=1 calls it in a new expression; USE=0 only
// declares it. Observation: does the program compile, link, and run?
using size_t = decltype(sizeof 0);
struct Arena { alignas(16) unsigned char buf[64]; };
void* operator new(size_t, Arena&);
struct S { int x; };
int main() {
#if USE
  Arena a;
  S* p = new (a) S{1};
  return p->x - 1;
#else
  return 0;
#endif
}
