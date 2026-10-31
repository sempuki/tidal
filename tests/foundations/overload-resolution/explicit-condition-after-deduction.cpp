// tidal-rules: OVL-16
// tidal-expect: accept
// tidal-note: The template deduces U = long, and explicit(true) then removes the specialization from copy-initialization. Without the removal the template would match exactly and win.
template <class T, class U> constexpr bool is = false;
template <class T> constexpr bool is<T, T> = true;
struct Value {
  int from = 0;
  template <class U> constexpr explicit(!is<U, int>) Value(U) : from(1) {}
  constexpr Value(double) : from(2) {}
};
constexpr Value a = 1L;
constexpr Value b = 1;
static_assert(a.from == 2);
static_assert(b.from == 1);
