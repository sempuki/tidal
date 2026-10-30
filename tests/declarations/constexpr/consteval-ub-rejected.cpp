// tidal-rules: CEV-1
// tidal-expect: reject
consteval int divide(int a, int b) { return a / b; }
int x = divide(1, 0);
