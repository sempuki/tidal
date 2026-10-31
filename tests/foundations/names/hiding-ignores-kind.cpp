// tidal-rules: NAM-12
// tidal-expect: reject
// tidal-error-clang: called object type .int. is not a function
// tidal-error-gcc: f. cannot be used as a function
// tidal-error-edg: apparent call must have \(pointer-to-\) function type
int f() { return 1; }
int main() { int f = 2; return f(); }
