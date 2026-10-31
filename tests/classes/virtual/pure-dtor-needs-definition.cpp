// tidal-rules: VIRT-9
// tidal-expect: link-fail
struct A { virtual ~A() = 0; };
struct B : A {};
int main() { B b; return 0; }
