// tidal-rules: NAM-7
// tidal-expect: reject
// tidal-error-clang: redefinition of label .a
// tidal-error-gcc: duplicate label .a
// tidal-error-edg: label .a. has already been defined
int main() { a: ; a: return 0; }
