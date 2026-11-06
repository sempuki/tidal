// tidal-rules: COND-16
// tidal-expect: accept
// tidal-note: Implement I4 (reports/clean-room/I4/report.md), finding I4-2. A const A lvalue against an A xvalue. Step 1 needs X to be an lvalue, so the xvalue does not bind const A&, and the result is a const A prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& mv(T& t) { return static_cast<T&&>(t); }
struct A { int v; };
A a{1};
const A ca{2};
bool c = true;
static_assert(same<decltype((c ? ca : mv(a))), const A>);
