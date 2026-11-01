// tidal-rules: CPT-27 CPT-32
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for class template
// tidal-error-gcc: template constraint failure
// tidal-error-edg: template constraint not satisfied
// tidal-note: Stack<int, 0> fails the constraint of Stack. The requirement depends on no template parameter, so the program is not valid, and the requires expression is not false.
template <class T, int N> requires (N > 0) struct Stack {};
static_assert(!requires { typename Stack<int, 0>; });
