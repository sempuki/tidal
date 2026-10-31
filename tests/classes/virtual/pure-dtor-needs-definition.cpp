// tidal-rules: VIRT-17
// tidal-expect: link-fail
struct A { virtual ~A() = 0; };
struct B : A {};
int main() { B b; return 0; }
