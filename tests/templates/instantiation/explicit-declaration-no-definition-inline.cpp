// tidal-rules: INST-28
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: run
// tidal-expect-clang-22-o2: run
// tidal-error: undefined reference to
// tidal-sources: explicit-declaration-no-definition-inline.tu.cpp
// tidal-note: The function is inline. No unit has an explicit instantiation definition. INST-28 allows any result. The -O0 builds fail to link, and the -O2 builds inline the body and run.
template <class T> inline int size(T value) { return sizeof(value); }
extern template int size(int);
int main() { return size(0) == sizeof(int) ? 0 : 1; }
