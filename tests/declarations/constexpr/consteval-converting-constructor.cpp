// tidal-rules: CEV-1
// tidal-expect: accept
// tidal-note: Write report W11. An implicit conversion through a consteval constructor is a consteval call. A throw on a path that the call does not reach is fine.
struct Identifier {
  const char* text;
  consteval Identifier(const char* s) : text(s) {
    if (s[0] >= '0' && s[0] <= '9') throw "starts with a digit";
  }
};
int length(Identifier id) { int n = 0; while (id.text[n]) ++n; return n; }
Identifier counter = "counter";
int use() { return length("total"); }
