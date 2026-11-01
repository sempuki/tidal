// tidal-rules: CPT-31
// tidal-expect: accept
// tidal-note: The expression sizeof(T) == 100 is valid and false. A simple requirement asks only that the expression is valid, so it holds.
template <class T> concept Valid = requires { sizeof(T) == 100; };
static_assert(Valid<char>);
