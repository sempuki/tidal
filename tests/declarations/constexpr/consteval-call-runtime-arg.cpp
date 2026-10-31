// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-error-clang: call to consteval function .square. is not a constant expression
// tidal-error-gcc: call to consteval function .square\(x\). is not a constant expression
// tidal-error-edg: call to consteval function .square. did not produce a valid constant expression
consteval int square(int v) { return v * v; }
int f(int x) { return square(x); }
