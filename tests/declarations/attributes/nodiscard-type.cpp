// tidal-rules: ATTR-5
// tidal-expect: reject
// tidal-flags: -Werror
struct [[nodiscard]] Error { int code; };
Error check() { return {0}; }
void use() { check(); }
