// tidal-rules: UNI-10
// tidal-expect: run
struct A { int tag; int a; };
struct B { int tag; double b; };
union U { A a; B b; };
int main() { U u; u.a.tag = 7; u.a.a = 1; return u.b.tag == 7 ? 0 : 1; }
