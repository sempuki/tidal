// tidal-rules: TR-3
// tidal-expect: reject
// tidal-error-clang: not allowed to be declared static
// tidal-error-gcc: to be static
// tidal-error-edg: .static. is not allowed
static int main() { return 0; }
