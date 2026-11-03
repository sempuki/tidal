// tidal-rules: CEV-1 CE-8
// tidal-expect: reject
// tidal-error-clang: call to consteval function .Identifier::Identifier. is not a constant expression
// tidal-error-gcc: <throw-expression>. is not a constant expression
// tidal-error-edg: call to consteval function "Identifier::Identifier\(const char \*\)" did not produce a valid constant expression
// tidal-note: Write report W11. The conversion of "2x" calls the consteval constructor, which reaches the throw, so the call is not a constant expression.
struct Identifier {
  const char* text;
  consteval Identifier(const char* s) : text(s) {
    if (s[0] >= '0' && s[0] <= '9') throw "starts with a digit";
  }
};
Identifier bad = "2x";
