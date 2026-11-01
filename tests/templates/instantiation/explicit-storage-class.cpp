// tidal-rules: INST-25
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: The explicit instantiation has the storage class static. Clang and EDG reject it, GCC accepts it (Q-0233).
// tidal-error-clang: explicit instantiation cannot have a storage class
// tidal-error-edg: a storage class may not be specified here
template <class T> int get(T) { return 1; }
template static int get(int);
