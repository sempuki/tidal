consteval bool ok(const char* s) { return *s != '2'; }
struct Identifier { const char* text;
  consteval Identifier(const char* s) : text(s) { if (!ok(s)) throw "not an identifier"; } };
constexpr Identifier bad = "2x";
int main() { return 0; }
