// tidal-rules: INST-26
// tidal-expect: reject
// tidal-error-clang: explicit instantiation declaration \(with 'extern'\) follows explicit instantiation definition
// tidal-error-gcc: duplicate explicit instantiation
// tidal-error-edg: "extern template" cannot follow explicit instantiation
template <class T> int get(T) { return 1; }
template int get(int);
extern template int get(int);
