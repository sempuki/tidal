// tidal-rules: CEV-2
// tidal-expect: reject
// tidal-error-clang: cannot take address of consteval function
// tidal-error-gcc: taking address of an immediate function
// tidal-error-edg: consteval|immediate
// tidal-note: The target type selects the consteval f(int), and the selection takes its address outside an immediate function context.
consteval int f(int v) { return v; }
int f(long v);
int (*p)(int) = f;
