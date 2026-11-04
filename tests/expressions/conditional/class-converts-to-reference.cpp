// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: A conversion to B& binds directly to the lvalue bl, so the result is an lvalue. A conversion to B gives no lvalue, so the result is a B prvalue. The xvalue cases follow the same steps with an rvalue reference.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& move(T& t) { return static_cast<T&&>(t); }
struct B { int m; };
struct ToRef { operator B&(); };
struct ToValue { operator B(); };
struct ToRvalueRef { operator B&&(); };
B bl;
extern bool c;
ToRef r;
ToValue v;
ToRvalueRef x;
static_assert(same<decltype((c ? r : bl)), B&>);
static_assert(same<decltype((c ? v : bl)), B>);
static_assert(same<decltype((c ? x : move(bl))), B&&>);
static_assert(same<decltype((c ? r : move(bl))), B>);
static_assert(same<decltype((c ? x : bl)), B>);
