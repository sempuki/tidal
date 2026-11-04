// tidal-rules: TR-3
// tidal-expect: reject
// tidal-error-clang: not allowed to be declared inline
// tidal-error-gcc: to be inline
// tidal-error-edg: may not be declared inline
inline int main() { return 0; }
