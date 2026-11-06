// tidal-rules: COND-3
// tidal-expect: accept
// tidal-note: Decision 0059, item F2 (I4-8). Two xvalues of one type apart from cv-qualifiers give an xvalue with the larger set. Two non-class xvalues whose sets do not include each other, or of different types, give a prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
struct Box { int v; };
Box box; const Box fixed{};
int i; const int ci = 0; volatile int vi; long l;
extern bool c;
static_assert(same<decltype(c ? move_of(box) : move_of(fixed)), const Box&&>);
static_assert(same<decltype(c ? move_of(i) : move_of(ci)), const int&&>);
static_assert(same<decltype(c ? move_of(ci) : move_of(vi)), int>);
static_assert(same<decltype(c ? move_of(i) : move_of(l)), long>);
