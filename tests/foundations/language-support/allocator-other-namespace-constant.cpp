// tidal-rules: LSUP-11 NEW-37
// tidal-expect: reject
// tidal-error-clang: cannot allocate untyped memory in a constant expression
// tidal-error-gcc: called in a constant expression
// tidal-error-edg: cannot call non-constexpr function .operator new
namespace pool {
using size_t = decltype(sizeof 0);
template <class T> struct allocator {
  constexpr allocator() noexcept {}
  constexpr T* allocate(size_t n) { return static_cast<T*>(::operator new(n * sizeof(T))); }
  constexpr void deallocate(T* p, size_t) { ::operator delete(p); }
};
}
constexpr int total() {
  pool::allocator<int> source;
  int* values = source.allocate(1);
  values[0] = 4;
  int v = values[0];
  source.deallocate(values, 1);
  return v;
}
static_assert(total() == 4);
