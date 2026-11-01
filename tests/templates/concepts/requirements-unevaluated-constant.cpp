// tidal-rules: CPT-30
// tidal-expect: accept
// tidal-note: The requirements hold a throw expression and a call of a function that is not constexpr. Neither is evaluated, so the concept-id is a constant.
int runtime();
template <class T> concept Quiet = requires (T t) { throw t; runtime(); };
static_assert(Quiet<int>);
