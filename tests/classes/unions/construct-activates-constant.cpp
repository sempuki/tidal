// tidal-rules: UNI-14
// tidal-expect: accept
// tidal-note: Point has no default constructor. std::construct_at creates a Point at the address of the member value, which has no active member before the call.
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
constexpr int fill() {
  Slot slot;
  std::construct_at(&slot.value, 3, 4);
  int sum = slot.value.x + slot.value.y;
  slot.value.~Point();
  return sum;
}
static_assert(fill() == 7);
