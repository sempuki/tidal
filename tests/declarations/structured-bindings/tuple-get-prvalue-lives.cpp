// tidal-rules: SB-17
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined,address -fno-sanitize-recover=undefined
// tidal-note: get returns a prvalue. Each name refers to a hidden reference that the result initializes, so each result lives until the names go out of scope.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
int alive = 0;
struct Token {
  int id;
  Token(int i) : id(i) { ++alive; }
  Token(const Token& o) : id(o.id) { ++alive; }
  ~Token() { --alive; }
};
struct Source {
  template <decltype(sizeof 0) I> Token get() const { return Token(int(I) + 1); }
};
template <> struct std::tuple_size<Source> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, Source> { using type = Token; };
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
int main() {
  int during = 0, sum = 0;
  {
    auto [a, b] = Source{};
    static_assert(same<decltype(a), Token>);
    during = alive;
    sum = a.id + b.id;
  }
  return during == 2 && sum == 3 && alive == 0 ? 0 : 1;
}
