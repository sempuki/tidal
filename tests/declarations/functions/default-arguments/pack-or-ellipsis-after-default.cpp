// tidal-rules: DEFARG-8
// tidal-expect: accept
// tidal-note: A function parameter pack and an ellipsis need no default argument after a parameter that has one.
template <class... T> constexpr int count(int first = 1, T... rest) { return first + sizeof...(rest); }
constexpr int variadic(int first = 2, ...) { return first; }
static_assert(count() == 1 && count(3, 'a', 2.0) == 5);
static_assert(variadic() == 2 && variadic(4) == 4);
