// tidal-rules: NAM-38
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: no operator .\+. matches these operands
// tidal-note: Q-0364. A block declares an operator+ that does not fit. Argument-dependent lookup still searches n and finds the operator+ that fits. A function call would not use argument-dependent lookup here (NAM-32). Clang and GCC accept. EDG does not use argument-dependent lookup and rejects.
namespace n { struct X {}; int operator+(X, X); }
struct Y {};
int g() { int operator+(Y, Y); n::X a; return a + a; }
