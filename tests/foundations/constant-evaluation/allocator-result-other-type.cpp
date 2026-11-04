// tidal-rules: CE-4
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: cast from .void \*. is not allowed in a constant expression
// tidal-note: allocate converts the result of ::operator new to long*, which is not the element type int. CE-4 allows only the conversion to int*. GCC and EDG accept the conversion (Q-0349).
namespace std {
using size_t = decltype(sizeof 0);
template <class T> struct allocator {
  constexpr allocator() noexcept {}
  constexpr long* allocate(size_t n) { return static_cast<long*>(::operator new(n * sizeof(T))); }
  constexpr void deallocate(long* p, size_t) { ::operator delete(p); }
};
}
constexpr int use() {
  std::allocator<int> source;
  long* p = source.allocate(2);
  source.deallocate(p, 2);
  return 1;
}
static_assert(use() == 1);
