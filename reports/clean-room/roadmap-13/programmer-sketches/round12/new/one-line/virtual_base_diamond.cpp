// The most derived class constructs the virtual base.
struct Base { int id; Base(int i) : id(i) {} };
struct L : virtual Base { L() : Base(1) {} };
struct R : virtual Base { R() : Base(2) {} };
struct D : L, R { D() : Base(3) {} };
int main() { D d; return d.L::id + d.R::id - 6; }
