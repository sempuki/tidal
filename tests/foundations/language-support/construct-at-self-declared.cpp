// tidal-rules: LSUP-10 STG-5
// tidal-expect: accept
inline void* operator new(decltype(sizeof 0), void* where) noexcept { return where; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
struct Span {
  int low, high;
  constexpr Span(int a, int b) : low(a), high(b) {}
};
constexpr int rebuild() {
  Span span{1, 2};
  span.~Span();
  Span* fresh = std::construct_at(&span, 10, 20);
  return fresh->low + span.high;
}
static_assert(rebuild() == 30);
