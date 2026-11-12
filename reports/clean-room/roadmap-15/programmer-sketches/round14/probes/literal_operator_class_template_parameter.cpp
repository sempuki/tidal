using size_t = decltype(sizeof 0);
template <size_t N> struct Str {
  char t[N]{};
  constexpr Str(const char (&s)[N]) { for (size_t i = 0; i < N; ++i) t[i] = s[i]; }
};
template <Str S> constexpr char operator""_first() { return S.t[0]; }
static_assert("xyz"_first == 'x');
int main() { return "a"_first - 'a'; }
