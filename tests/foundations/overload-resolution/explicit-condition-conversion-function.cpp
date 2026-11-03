// tidal-rules: OVL-16
// tidal-expect: accept
// tidal-note: The conversion function template deduces U = long, and explicit(true) then removes the specialization from copy-initialization, so the conversion to double is used. For int, the template is not explicit and wins by its exact match.
template <class T, class U> constexpr bool is = false;
template <class T> constexpr bool is<T, T> = true;
struct Source {
  template <class U> constexpr explicit(!is<U, int>) operator U() const { return U(1); }
  constexpr operator double() const { return 2; }
};
constexpr long a = Source{};
constexpr int b = Source{};
static_assert(a == 2);
static_assert(b == 1);
