// tidal-rules: PACK-20
// tidal-expect: reject
// tidal-error-clang: expected '\)'
// tidal-error-gcc: expected .\). before .\*. token
// tidal-error-edg: expected a "\)"
// tidal-note: The operand must be the name of a pack. T* is a pattern.
template <class... T> constexpr auto count = sizeof...(T*);
