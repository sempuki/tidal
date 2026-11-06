// tidal-rules: COND-16
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: Implement I4 (reports/clean-room/I4/report.md), finding I4-2. Q-0418. A Derived xvalue against a const Base lvalue. Step 1 needs X to be an lvalue, so the result is a const Base prvalue, as GCC gives. Clang binds the xvalue and gives an lvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& mv(T& t) { return static_cast<T&&>(t); }
struct Base { int v; };
struct Derived : Base { };
Derived d{};
const Base cb{7};
bool c = true;
static_assert(same<decltype((c ? mv(d) : cb)), const Base>);
