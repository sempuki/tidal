// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: Decision 0059, item F2 (I4-13). A class prvalue with a conversion function to int&& meets an int xvalue. Step 2 binds int&& directly to the xvalue that the conversion function returns, so the result is an int xvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
int store;
struct Source {
  operator int&&() { return move_of(store); }
};
int other;
extern bool c;
static_assert(same<decltype(c ? Source{} : move_of(other)), int&&>);
