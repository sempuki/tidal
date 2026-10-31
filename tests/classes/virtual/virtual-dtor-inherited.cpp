// tidal-rules: VIRT-7
// tidal-expect: run
int count = 0;
struct A { virtual ~A() {} };
struct B : A {};
struct D : B { ~D() { ++count; } };
int main() { A* p = new D; delete p; return count == 1 ? 0 : 1; }
