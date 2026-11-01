// tidal-rules: TPL-44
// tidal-expect: run
// tidal-note: The specialization Derived::use<int> has the type of Base::use, but it does not override it.
struct Base { virtual int use(int) { return 1; } };
struct Derived : Base { template <class T> int use(T) { return 2; } };
int main() { Derived d; Base& b = d; return b.use(0) == 1 && d.use(0) == 2 ? 0 : 1; }
