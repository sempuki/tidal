// tidal-rules: MPTR-5
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int main() { int S::* p = &S::a; int S::* q = &S::a; int S::* r = &S::b; return p == q && p != r && r != nullptr ? 0 : 1; }
