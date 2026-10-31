// tidal-rules: ODR-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .total
// tidal-error-gcc: redefinition of .int total
// tidal-error-edg: has already been defined
int total = 1;
int total = 2;
int main() { return 0; }
