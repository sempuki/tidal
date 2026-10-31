// tidal-rules: ODR-9
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-expect-clang-18: run
// tidal-error-edg: referenced but not defined
// tidal-error-clang: has internal linkage but is not defined
// tidal-note: Q-0128. GCC does not translate the unused inline function and gives no diagnostic. Clang 18 gives a warning only.
static int hidden();
inline int unused_caller() { return hidden(); }
int main() { return 0; }
