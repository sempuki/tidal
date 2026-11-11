// tidal-rules: OPR-8 OPR-10
// tidal-expect: accept
// tidal-note: Gap review RV8-A-3. A literal operator template whose one template parameter is a constant of class type, with an empty parameter list, is valid. A string literal with the suffix calls it, with the string literal as the template argument. The parameter can name a class template whose arguments the string literal deduces. Every compiler accepts both forms.
using size = decltype(sizeof 0);
struct Name {
  char data[4];
  constexpr Name(const char (&text)[4]) : data{text[0], text[1], text[2], text[3]} {}
};
template <Name N> constexpr char operator""_second() { return N.data[1]; }
static_assert("abc"_second == 'b');
template <size N> struct Text {
  char data[N];
  constexpr Text(const char (&text)[N]) : data{} {
    for (size i = 0; i < N; ++i) data[i] = text[i];
  }
};
template <Text T> constexpr size operator""_length() { return sizeof(T.data) - 1; }
static_assert("hello"_length == 5);
