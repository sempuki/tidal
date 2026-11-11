// tidal-rules: NEW-25 NEW-13
// tidal-expect: reject
// tidal-error-clang: call to deleted function .operator new.
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: G155 (front-end report 23). The class declares its own operator new as deleted, so the new expression selects a deleted allocation function (NEW-13), which NEW-25 forbids.
using size = decltype(sizeof 0);
struct Pinned {
  static void* operator new(size) = delete;
  static void operator delete(void* p) { ::operator delete(p); }
};
int main() {
  Pinned* p = new Pinned;
  delete p;
  return 0;
}
