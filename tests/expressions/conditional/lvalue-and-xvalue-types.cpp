// tidal-rules: COND-3 COND-16
// tidal-expect: accept
// tidal-note: Decision 0059, item F2 (I4-8). An lvalue and an xvalue of one type give a prvalue of that type. A non-class pair that differs in cv-qualifiers loses them, and a class pair goes to the last row of COND-3, where only the operand toward the more qualified type converts.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
struct Box { int v; };
Box box; const Box fixed{};
int i; const int ci = 0; long l;
extern bool c;
static_assert(same<decltype(c ? i : move_of(i)), int>);
static_assert(same<decltype(c ? i : move_of(ci)), int>);
static_assert(same<decltype(c ? i : move_of(l)), long>);
static_assert(same<decltype(c ? box : move_of(box)), Box>);
static_assert(same<decltype(c ? box : move_of(fixed)), const Box>);
static_assert(same<decltype(c ? fixed : move_of(box)), const Box>);
