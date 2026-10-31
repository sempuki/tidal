// tidal-rules: ATTR-4
// tidal-expect: reject
// tidal-error-clang: old. is deprecated
// tidal-error-gcc: old\(\). is deprecated
// tidal-flags: -Werror
// tidal-note: Gap review 2026-10-31.
void old();
[[deprecated]] void old();
void use() { old(); }
