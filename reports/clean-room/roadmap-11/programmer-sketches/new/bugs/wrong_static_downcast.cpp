// A static_cast from a base reference to the wrong derived class.
struct Base { int tag = 0; virtual ~Base() = default; };
struct A : Base { int a = 1; };
struct B : Base { int b = 2; };
int read(Base& x) { return static_cast<B&>(x).b; }
int main() { A a; return read(a) == 2 ? 0 : 1; }
