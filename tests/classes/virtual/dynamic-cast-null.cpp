// tidal-rules: VIRT-10
// tidal-expect: run
struct B { virtual ~B() {} };
struct D : B {};
int main() { B* p = nullptr; return dynamic_cast<D*>(p) == nullptr ? 0 : 1; }
