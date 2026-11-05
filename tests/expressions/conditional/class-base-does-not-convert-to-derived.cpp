// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: Derived has a constructor from Base, so a Base converts to a Derived in other places. In the conditional operator the Base operand does not convert to its derived class. Only the Derived operand converts, to Base, and the result is a Base prvalue. Clang 22, GCC 16, and EDG 7.0 agree.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Base { Base(); };
struct Derived : Base { Derived(); Derived(const Base&); };
Derived make_derived();
Base make_base();
extern bool c;
static_assert(same<decltype(c ? make_base() : make_derived()), Base>);
