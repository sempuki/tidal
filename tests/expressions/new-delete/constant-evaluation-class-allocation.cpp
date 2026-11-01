// tidal-rules: NEW-37
// tidal-expect: reject
// tidal-error-clang: call to class-specific .operator new
// tidal-error-gcc: call to non-.constexpr. function .static void\* Own::operator new
// tidal-error-edg: class-specific allocation function cannot be constant-evaluated
using size = decltype(sizeof 0);
alignas(16) unsigned char pool[64];
struct Own {
  int value;
  static void* operator new(size) { return pool; }
  static void operator delete(void*) noexcept {}
};
constexpr int make() {
  Own* p = new Own{3};
  int v = p->value;
  delete p;
  return v;
}
static_assert(make() == 3);
