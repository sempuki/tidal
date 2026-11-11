// tidal-rules: NEW-25 NEW-13
// tidal-expect: reject
// tidal-error-clang: call to deleted function .operator new\[\].
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: G155 (front-end report 23). The class declares operator new[] as deleted, so the array new expression selects a deleted allocation function (NEW-13, NEW-25). The single-object form stays usable.
using size = decltype(sizeof 0);
struct Pinned {
  static void* operator new(size n) { return ::operator new(n); }
  static void* operator new[](size) = delete;
  static void operator delete(void* p) { ::operator delete(p); }
  static void operator delete[](void* p) { ::operator delete(p); }
};
int main() {
  Pinned* one = new Pinned;
  delete one;
  Pinned* many = new Pinned[2];
  delete[] many;
  return 0;
}
