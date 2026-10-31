// tidal-rules: VIRT-7
// tidal-expect: run
int count = 0;
struct B { virtual ~B() {} };
struct D : B { ~D() { ++count; } };
int main() { B* p = new D; delete p; return count == 1 ? 0 : 1; }
