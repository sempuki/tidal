// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: attribute .nodiscard. does not apply here
// tidal-note: Gap review 2026-10-30. Q-0021. Clang and GCC ignore nodiscard on a variable with a warning; EDG rejects it.
[[nodiscard]] int x;
