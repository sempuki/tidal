// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: noreturn. attribute does not appear on the first declaration
// tidal-error-gcc: but its first declaration was not
// tidal-error-edg: did not appear on original declaration
// tidal-note: Gap review 2026-10-30.
void stop();
[[noreturn]] void stop();
