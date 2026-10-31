// tidal-rules: DCL-16
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-edg: accept
// tidal-error-clang: reference to function type .* cannot have .&. qualifier
// tidal-error-gcc: cannot declare reference to qualified function type
// tidal-note: Q-0106. EDG accepts a reference parameter to a function type with a ref-qualifier. Clang and GCC reject it.
using F = void() &;
void take(F& f);
