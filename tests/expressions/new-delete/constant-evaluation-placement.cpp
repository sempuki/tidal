// tidal-rules: NEW-37
// tidal-expect: reject
// tidal-error-clang: call to placement .operator new|placement new expression is not supported
// tidal-error-gcc: call to non-.constexpr. function .void\* operator new\(size, void\*\)
// tidal-error-edg: placement new-expression cannot be constant-evaluated
using size = decltype(sizeof 0);
inline void* operator new(size, void* where) noexcept { return where; }
constexpr int make() {
  int x = 1;
  int* p = new (&x) int(4);
  return *p;
}
static_assert(make() == 4);
