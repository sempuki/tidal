// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: fallthrough. attribute cannot be applied to a declaration
// tidal-error-gcc: fallthrough. attribute ignored
// tidal-error-edg: attribute .fallthrough. does not apply here
[[fallthrough]] int x;
