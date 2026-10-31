// tidal-rules: VIRT-9
// tidal-expect: run
int count = 0;
struct A { virtual ~A() = 0; };
A::~A() { ++count; }
struct B : A {};
int main() { { B b; } return count == 1 ? 0 : 1; }
