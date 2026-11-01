// tidal-rules: DED-27 DED-29
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to
// tidal-error-gcc: is not a member of
// tidal-error-edg: must be a class or namespace name
// tidal-note: The noexcept operand is not part of substitution, so the template stays a candidate and wins. The operand is then not valid for int.
template <class T> void act(T) noexcept(T::value) {}
void act(long) {}
void use() { act(1); }
