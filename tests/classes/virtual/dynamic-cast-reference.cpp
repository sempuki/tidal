// tidal-rules: VIRT-10
// tidal-expect: run
struct B { virtual ~B() {} };
struct D : B { int v = 3; };
int main() { D d; B& r = d; return dynamic_cast<D&>(r).v == 3 ? 0 : 1; }
