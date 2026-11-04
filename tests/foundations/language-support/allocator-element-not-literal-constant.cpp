// tidal-rules: LSUP-11
// tidal-expect: accept
// tidal-note: Plain has a destructor that is not constexpr, so Plain is not a literal type. Every compiler runs allocate and deallocate of std::allocator<Plain> during constant evaluation. The gap review of the object model reads the C++23 text the other way (documentation/decisions/pending.md, row 4).
namespace std {
using size_t = decltype(sizeof 0);
template <class T> struct allocator {
  constexpr allocator() noexcept {}
  constexpr T* allocate(size_t n) { return static_cast<T*>(::operator new(n * sizeof(T))); }
  constexpr void deallocate(T* p, size_t) { ::operator delete(p); }
};
}
struct Plain { int value; ~Plain() {} };
constexpr bool round_trip() {
  std::allocator<Plain> source;
  Plain* items = source.allocate(2);
  source.deallocate(items, 2);
  return true;
}
static_assert(round_trip());
