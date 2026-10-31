// tidal-rules: DER-16
// tidal-expect: run
struct B { int b = 1; };
struct D : B { int d = 2; };
int main() { D d; B& r = d; B* p = &d; return static_cast<D&>(r).d == 2 && static_cast<D*>(p)->d == 2 ? 0 : 1; }
