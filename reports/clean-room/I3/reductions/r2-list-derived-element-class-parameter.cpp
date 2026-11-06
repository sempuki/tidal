struct Base { Base() {} };
struct Derived : Base { Derived() {} };
Derived d;
int take(Base) { return 0; }
int main() { return take({d}); }
