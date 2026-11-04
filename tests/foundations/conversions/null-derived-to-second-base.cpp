// tidal-rules: CONV-14
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct A { int a; };
struct B { int b; };
struct D : A, B {};
int main() { D* d = nullptr; B* b = d; return b == nullptr ? 0 : 1; }
