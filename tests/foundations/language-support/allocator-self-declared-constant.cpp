// tidal-rules: LSUP-11
// tidal-expect: accept
inline constexpr void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
namespace std {
using size_t = decltype(sizeof 0);
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
template <class T> struct allocator {
  constexpr allocator() noexcept {}
  constexpr T* allocate(size_t n) { return static_cast<T*>(::operator new(n * sizeof(T))); }
  constexpr void deallocate(T* p, size_t) { ::operator delete(p); }
};
}
constexpr int total() {
  std::allocator<int> source;
  int* values = source.allocate(3);    // storage for an int[3]
  for (int i = 0; i != 3; ++i) std::construct_at(values + i, i + 1);
  int sum = values[0] + values[1] + values[2];
  source.deallocate(values, 3);
  return sum;
}
static_assert(total() == 6);
