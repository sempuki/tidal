// tidal-rules: COND-11 COND-16
// tidal-expect: run
// tidal-note: Implement I4 (reports/clean-room/I4/report.md), finding I4-3. A const A lvalue and a volatile A lvalue: neither converts to the other's type, so COND-11 tries the built-in candidates, and A's conversion to int gives an int.
struct A { operator int() const volatile { return 3; } };
const A ca{};
volatile A va{};
int main() { bool c = true; int x = c ? ca : va; return x == 3 ? 0 : 1; }
