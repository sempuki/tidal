// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: Both operands have class type Box apart from cv-qualifiers. The prvalue converts toward the const lvalue, whose type has at least its cv-qualifiers. The const lvalue does not convert toward plain Box, so only one conversion succeeds, and the result is a const Box prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Box { };
const Box fixed{};
Box make();
extern bool c;
static_assert(same<decltype(c ? fixed : make()), const Box>);
static_assert(same<decltype(c ? make() : fixed), const Box>);
