// tidal-rules: DCL-19
// tidal-expect: reject
// tidal-error-clang: value of type .N. is not implicitly convertible
// tidal-error-gcc: has non-integral type .N|could not convert .N::Three.
// tidal-error-edg: has type .N. instead of the required
// tidal-note: Gap review 2026-10-30.
enum class N { Three = 3 };
int a[N::Three];
