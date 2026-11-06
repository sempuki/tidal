// tidal-rules: COND-16
// tidal-expect: run
// tidal-note: Implement I4 (reports/clean-room/I4/report.md), finding I4-1. A Derived prvalue against a const Base lvalue. The base row's target is T, const Base, because T has every cv-qualifier of the Derived operand. So the result is a const Base prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Base { int v; };
struct Derived : Base { };
const Base cb{7};
bool c = false;
static_assert(same<decltype((c ? Derived{} : cb)), const Base>);
int main() { Base r = c ? Derived{} : cb; return r.v == 7 ? 0 : 1; }
