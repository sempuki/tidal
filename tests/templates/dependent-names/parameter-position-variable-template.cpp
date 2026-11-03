// tidal-rules: DEP-29 DEP-19
// tidal-expect: accept
// tidal-note: At namespace scope, T::type is a value (DEP-19), so take(T::type) declares a variable template with a parenthesized initializer.
struct Limit { static constexpr int type = 3; };
template <class T> constexpr int take(T::type);
static_assert(take<Limit> == 3);
