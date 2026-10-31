// tidal-rules: DER-5
// tidal-expect: run
struct A { int a; };
struct B { int b; };
struct D : A, B {};
int main() { D* d = nullptr; B* p = d; return p == nullptr ? 0 : 1; }
