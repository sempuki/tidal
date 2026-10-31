// tidal-rules: ATTR-5
// tidal-expect: reject
// tidal-error-clang: declared with .nodiscard. attribute
// tidal-error-gcc: value of type .Error., declared with attribute .nodiscard
// tidal-flags: -Werror
struct [[nodiscard]] Error { int code; };
Error check() { return {0}; }
void use() { check(); }
