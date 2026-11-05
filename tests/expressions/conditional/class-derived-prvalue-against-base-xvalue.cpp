// tidal-rules: COND-16
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: G148. A Derived prvalue against a Base xvalue. A reference does not bind directly to a class prvalue, so the result is a prvalue of Base.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> T&& move_(T& t) { return static_cast<T&&>(t); }
struct Base {};
struct Derived : Base {};
Base b;
bool c = true;
static_assert(same<decltype(c ? Derived{} : move_(b)), Base>);
