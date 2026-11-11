// tidal-rules: OPR-8
// tidal-expect: accept
// tidal-note: Gap review RV8-A-3. "abcd" has five characters and is not a valid argument for Name, whose constructor takes four, so the template does not apply and the string literal calls the string form. Every compiler agrees.
using size = decltype(sizeof 0);
struct Name {
  char data[4];
  constexpr Name(const char (&text)[4]) : data{text[0], text[1], text[2], text[3]} {}
};
template <Name N> constexpr int operator""_tag() { return 1; }
constexpr int operator""_tag(const char*, size) { return 2; }
static_assert("abcd"_tag == 2);
