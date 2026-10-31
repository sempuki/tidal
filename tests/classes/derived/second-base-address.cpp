// tidal-rules: DER-5
// tidal-expect: run
struct A { int a = 1; };
struct B { int b = 2; };
struct D : A, B {};
int main() { D d; B* p = &d; return p->b == 2 && p == &static_cast<B&>(d) ? 0 : 1; }
