// tidal-rules: UNI-14 UNI-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: outside its lifetime
// tidal-error-edg: expression must have a constant value|uninitialized|lifetime
// tidal-note: Q-0309. After the explicit destructor call the union has no active member, so the read is undefined. GCC 16 reads the value 0 during constant evaluation.
inline constexpr void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
struct Point {
  int x, y;
  constexpr Point(int a, int b) : x(a), y(b) {}
};
union Slot {
  Point value;
  constexpr Slot() {}
  constexpr ~Slot() {}
};
constexpr int read_after_end() {
  Slot slot;
  std::construct_at(&slot.value, 3, 4);
  slot.value.~Point();
  return slot.value.x;
}
constexpr int value = read_after_end();
