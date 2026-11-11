// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-error-clang: template parameter list for literal operator must be
// tidal-error-gcc: has invalid parameter list
// tidal-error-edg: a literal operator template must have
// tidal-note: Gap review RV8-A-3. A literal operator template has one template parameter, either a pack of char or a constant of class type. A head with two class-type parameters is not valid.
struct Name {
  char data[4];
  constexpr Name(const char (&text)[4]) : data{text[0], text[1], text[2], text[3]} {}
};
template <Name N, Name M> constexpr int operator""_tag() { return 1; }
