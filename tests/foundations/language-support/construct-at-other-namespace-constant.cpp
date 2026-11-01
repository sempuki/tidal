// tidal-rules: NEW-37 LSUP-10
// tidal-expect: reject
// tidal-error-clang: call to placement .operator new|placement new expression is not supported
// tidal-error-gcc: call to non-.constexpr. function .void\* operator new
// tidal-error-edg: placement new-expression cannot be constant-evaluated
inline void* operator new(decltype(sizeof 0), void* where) noexcept { return where; }
namespace tools {
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
  Span* fresh = tools::construct_at(&span, 10, 20);
  return fresh->low + span.high;
}
static_assert(rebuild() == 30);
