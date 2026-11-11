// tidal-rules: OPR-8
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Gap review RV8-A-3. For _tag, a literal operator template with one class-type parameter and the string form both exist, and "abc" is a valid argument for Name, so the string literal calls the template, as Clang does. GCC and EDG call the string form (Q-0446).
using size = decltype(sizeof 0);
struct Name {
  char data[4];
  constexpr Name(const char (&text)[4]) : data{text[0], text[1], text[2], text[3]} {}
};
template <Name N> constexpr int operator""_tag() { return 1; }
constexpr int operator""_tag(const char*, size) { return 2; }
static_assert("abc"_tag == 1);
