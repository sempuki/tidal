// tidal-rules: ODR-11 ODR-9
// tidal-expect: reject
// tidal-error-clang: has internal linkage but is not defined
// tidal-error-gcc: used but never defined
// tidal-flags: -Werror
static int hidden();
int main() { return hidden(); }
