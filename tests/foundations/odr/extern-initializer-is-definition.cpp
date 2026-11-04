// tidal-rules: ODR-15 ODR-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .total
// tidal-error-gcc: redefinition of .int total
// tidal-error-edg: has already been defined
// tidal-note: extern with an initializer defines total, and int total; without extern defines it again.
extern int total = 1;
int total;
int main() { return 0; }
