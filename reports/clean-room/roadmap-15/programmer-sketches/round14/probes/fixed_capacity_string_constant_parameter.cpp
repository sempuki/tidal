using size_t = decltype(sizeof 0);
struct Str { char t[16]{}; size_t n = 0; constexpr Str(const char* s) { while (s[n]) { t[n] = s[n]; ++n; } } };
template <Str S> struct Key { static constexpr auto name = S; };
static_assert(Key<"b">::name.t[0] == 'b');
template <class K> constexpr int f(K) { return K::name.t[0]; }
static_assert(f(Key<"c">{}) == 'c');
int main() {}
