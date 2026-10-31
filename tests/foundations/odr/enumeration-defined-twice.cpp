// tidal-rules: ODR-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .Color
// tidal-error-gcc: multiple definition of .enum
// tidal-error-edg: has already been defined
enum Color { red, green };
enum Color { blue, black };
int main() { return 0; }
