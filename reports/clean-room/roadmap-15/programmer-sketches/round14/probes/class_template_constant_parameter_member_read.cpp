using size_t = decltype(sizeof 0);
template <size_t N> struct FS { char t[N]{}; constexpr FS(const char (&s)[N]) { for (size_t i = 0; i < N; ++i) t[i] = s[i]; } };
template <FS S> struct Key { static constexpr int first = S.t[0]; };
static_assert(Key<"b">::first == 'b');
int main() {}
