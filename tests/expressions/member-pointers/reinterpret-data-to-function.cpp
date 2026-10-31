// tidal-rules: MPTR-4
// tidal-expect: reject
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
auto pf = reinterpret_cast<int (S::*)()>(&S::a);
