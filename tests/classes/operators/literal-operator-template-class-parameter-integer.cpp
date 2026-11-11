// tidal-rules: OPR-16
// tidal-expect: reject
// tidal-error-clang: no matching literal operator
// tidal-error-gcc: no matching function for call
// tidal-error-edg: user-defined
// tidal-note: Gap review RV8-A-3. A literal operator template with a class-type parameter serves string literals only. For an integer literal, no literal operator for _tag matches.
struct Name {
  char data[4];
  constexpr Name(const char (&text)[4]) : data{text[0], text[1], text[2], text[3]} {}
};
template <Name N> constexpr int operator""_tag() { return 1; }
int x = 12_tag;
