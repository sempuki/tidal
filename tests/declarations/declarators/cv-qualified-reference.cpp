// tidal-rules: DCL-18
// tidal-expect: reject
// tidal-error-clang: qualifier may not be applied to a reference
// tidal-error-gcc: qualifiers cannot be applied to .int&
// tidal-error-edg: type qualifier on a reference type is not allowed
int x;
int& const r = x;
