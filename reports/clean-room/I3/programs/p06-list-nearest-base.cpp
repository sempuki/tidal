// ICS-19 third bullet with ICS-36: {d} has the element's sequence.
struct Base { Base() {} };
struct Middle : Base { Middle() {} };
struct Derived : Middle { Derived() {} };
Derived d;
int take(Middle) { return 1; }
int take(Base) { return 2; }
int main() { return take({d}) == 1 ? 0 : 1; }
