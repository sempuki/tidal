// tidal-rules: NEW-13 NEW-25
// tidal-expect: run
// tidal-note: G155 (front-end report 23). The class's own operator new is private. The expression ::new uses the global allocation function (NEW-13), which is accessible, so NEW-25 does not reach the private one.
using size = decltype(sizeof 0);
int calls = 0;
struct Pinned {
  int v = 3;
private:
  static void* operator new(size n) { ++calls; return ::operator new(n); }
};
int main() {
  Pinned* p = ::new Pinned;
  int v = p->v;
  ::delete p;
  return v == 3 && calls == 0 ? 0 : 1;
}
