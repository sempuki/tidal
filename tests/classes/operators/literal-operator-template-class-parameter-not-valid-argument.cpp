// tidal-rules: OPR-16
// tidal-expect: reject
// tidal-error-clang: no matching literal operator
// tidal-error-gcc: no matching function for call
// tidal-expect-edg: accept
// tidal-note: Gap review RV8-A-3. Only the literal operator template exists for _tag, and "abcd" is not a valid argument for Name, so no literal operator matches the literal. EDG accepts the program (Q-0446).
struct Name {
  char data[4];
  constexpr Name(const char (&text)[4]) : data{text[0], text[1], text[2], text[3]} {}
};
template <Name N> constexpr int operator""_tag() { return 1; }
int x = "abcd"_tag;
