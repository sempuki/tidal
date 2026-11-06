// const A lvalue against A xvalue: COND-16 step 1 binds const A& to the xvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& mv(T& t) { return static_cast<T&&>(t); }
struct A { int v; };
A a{1};
const A ca{2};
bool c = true;
static_assert(same<decltype((c ? ca : mv(a))), const A&>);
int main() { const A& r = c ? ca : mv(a); return &r == &ca ? 0 : 1; }
