// tidal-rules: INST-25
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: The explicit instantiation has an attribute after the keyword template. Clang and EDG reject it, GCC accepts it (Q-0233).
// tidal-error-clang: an attribute list cannot appear here
// tidal-error-edg: attributes may not appear here
template <class T> int get(T) { return 1; }
template [[nodiscard]] int get(int);
