// tidal-rules: UNI-14 LSUP-10
// tidal-expect: accept
// tidal-note: The construction at the address of label makes label the active member in place of count.
inline constexpr void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
struct Label {
  char first;
  constexpr explicit Label(char c) : first(c) {}
};
union Cell {
  int count;
  Label label;
};
constexpr char relabel() {
  Cell cell{.count = 1};
  std::construct_at(&cell.label, 'z');
  return cell.label.first;
}
static_assert(relabel() == 'z');
