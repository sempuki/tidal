using size_t = decltype(sizeof 0);
template <size_t N> struct Str {
  char t[N]{};
  constexpr Str(const char (&s)[N]) { for (size_t i = 0; i < N; ++i) t[i] = s[i]; }
};
template <Str S> struct Key { static constexpr char first = S.t[0]; };
static_assert(Key<"b">::first == 'b');
int main() { return Key<"a">::first - 'a'; }
