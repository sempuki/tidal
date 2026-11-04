// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: The other operand's type is a base of the operand's class. The target is that base with the derived operand's cv-qualifiers, and the base type must have at least those cv-qualifiers.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Base { };
struct Derived : Base { };
Derived plain{};
const Derived fixed{};
Base make();
const Base make_const();
extern bool c;
static_assert(same<decltype(c ? plain : make()), Base>);
static_assert(same<decltype(c ? fixed : make_const()), const Base>);
