// tidal-rules: INL-6
// tidal-expect: link-fail
// tidal-expect-edg: reject
// tidal-error-edg: inline function .missing. was referenced but not defined
// tidal-note: EDG reports the missing definition during translation. Clang and GCC report it when linking.
inline int missing();
int main() { return missing(); }
