// tidal-rules: DER-3
// tidal-expect: run
struct B { struct T { int v = 5; }; };
struct D : B { int T = 6; };
int main() { D d; B::T t; return d.T + t.v == 11 ? 0 : 1; }
