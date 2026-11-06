// Derived xvalue against const Base lvalue: is the result an lvalue?
template <class T> T&& mv(T& t) { return static_cast<T&&>(t); }
struct Base { int v; };
struct Derived : Base { };
Derived d{};
const Base cb{7};
int main() { bool c = true; const Base& r = c ? mv(d) : cb; return &r == &d ? 0 : 1; }
