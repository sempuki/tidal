// tidal-rules: VIRT-17
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .A::~A\(\)
// tidal-error-gcc: undefined reference to .A::~A\(\)
struct A { virtual ~A() = 0; };
struct B : A {};
int main() { B b; return 0; }
