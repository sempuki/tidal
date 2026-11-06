// Derived xvalue against Base lvalue and const Base lvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& mv(T& t) { return static_cast<T&&>(t); }
struct Base { int v; };
struct Derived : Base { };
Derived d{};
Base b{};
const Base cb{};
bool c = true;
static_assert(same<decltype((c ? mv(d) : b)), Base>);
static_assert(same<decltype((c ? mv(d) : cb)), const Base&>);
int main() { const Base& r = c ? mv(d) : cb; return &r == &d ? 0 : 1; }
