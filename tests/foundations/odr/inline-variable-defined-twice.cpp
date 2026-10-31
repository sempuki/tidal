// tidal-rules: ODR-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .limit
// tidal-error-gcc: redefinition of .int limit
// tidal-error-edg: has already been defined
inline int limit = 1;
inline int limit = 1;
int main() { return 0; }
