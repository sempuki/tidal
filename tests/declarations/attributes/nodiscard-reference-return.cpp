// tidal-rules: ATTR-5
// tidal-expect: accept
// tidal-flags: -Wall -Werror
// tidal-note: Gap review 2026-10-31.
struct [[nodiscard]] Result { int code; };
Result r;
Result& get() { return r; }
void use() { get(); }
