// A class type as a constant template parameter, built from a string literal.
template <unsigned N> struct Name { char s[N]; constexpr Name(const char (&a)[N]) { for (unsigned i = 0; i < N; ++i) s[i] = a[i]; } };
template <Name N> constexpr unsigned len() { return sizeof(N.s) - 1; }
int main() { return static_cast<int>(len<"hello">()) - 5; }
