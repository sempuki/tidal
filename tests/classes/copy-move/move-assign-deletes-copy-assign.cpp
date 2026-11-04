// tidal-rules: CPY-21 CPY-4
// tidal-expect: reject
// tidal-error-gcc: deleted function
// tidal-error-clang: implicitly deleted
// tidal-error-edg: deleted
struct Token {
  int id;
  Token(int v) : id(v) {}
  Token& operator=(Token&& from) { id = from.id; return *this; }
};
void reset(Token& t, Token& source) { t = source; }
