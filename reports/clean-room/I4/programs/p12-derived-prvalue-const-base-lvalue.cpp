// Derived prvalue against const Base lvalue. COND-16's base row gives a
// Base prvalue (the cv-qualifiers of X), which then differs from const Base.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Base { int v; };
struct Derived : Base { };
const Base cb{7};
bool c = false;
static_assert(same<decltype((c ? Derived{} : cb)), const Base>);
int main() { Base r = c ? Derived{} : cb; return r.v == 7 ? 0 : 1; }
