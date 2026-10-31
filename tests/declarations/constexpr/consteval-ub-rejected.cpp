// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-error-clang: call to consteval function .divide. is not a constant expression
// tidal-error-gcc: call to consteval function .divide\(1, 0\). is not a constant expression
// tidal-error-edg: call to consteval function .divide. did not produce a valid constant expression
consteval int divide(int a, int b) { return a / b; }
int x = divide(1, 0);
