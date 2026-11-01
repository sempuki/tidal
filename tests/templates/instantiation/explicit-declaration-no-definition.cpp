// tidal-rules: INST-28
// tidal-expect: link-fail
// tidal-error: undefined reference to
// tidal-sources: explicit-declaration-no-definition.tu.cpp
// tidal-note: The function is not inline. No unit has an explicit instantiation definition. INST-28 allows any result. Every compiler that links reports the missing definition.
template <class T> int size(T value) { return sizeof(value); }
extern template int size(int);
int main() { return size(0) == sizeof(int) ? 0 : 1; }
