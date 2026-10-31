// tidal-rules: NEW-20 NEW-10
// tidal-expect: run
struct arena { int used = 0; };
struct view { arena* where; view(arena& a) : where(&a) { } };
alignas(int) unsigned char pool[64];
void* operator new(decltype(sizeof 0), view v) { v.where->used = 1; return pool; }
int main() {
  arena a;
  int* p = new (a) int(5);
  return a.used == 1 && *p == 5 ? 0 : 1;
}
