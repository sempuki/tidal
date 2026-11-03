// tidal-rules: NEW-18
// tidal-expect: run
// tidal-note: The allocation function is a member of the class. Either order of the placement argument and the bound is allowed, and both come before the call.
int order[3];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
alignas(16) unsigned char pool[128];
struct Cell {
  int v = 0;
  static void* operator new[](decltype(sizeof 0), int) { mark(3); return pool; }
  static void operator delete[](void*) noexcept {}
};
int main() {
  Cell* p = new (mark(1)) Cell[mark(2)];
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return either && order[2] == 3 && p[1].v == 0 ? 0 : 1;
}
