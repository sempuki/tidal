// tidal-rules: ATTR-13
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: assume. attribute cannot be applied to a declaration
// tidal-error-gcc: assume. attribute ignored
// tidal-error-edg: attribute .assume. does not apply here
// tidal-note: Q-0020. Clang 18 doesn't implement [[assume]] and ignores it as an unknown attribute.
[[assume(true)]] int x;
