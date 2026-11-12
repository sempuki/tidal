// tidal-rules: PRIM-28 OPR-6
// tidal-expect: accept
// tidal-note: X26-5. Each element of the pack has a class type, so the comma in each fold selects the overloaded operator. The right fold nests to the right and the left fold to the left. Every compiler calls the operator in both folds.
struct Digit { int value; };
constexpr Digit operator,(Digit left, Digit right) { return {left.value * 10 + right.value}; }
template <class... D> constexpr int right_fold(D... d) { return (d, ...).value; }
template <class... D> constexpr int left_fold(D... d) { return (..., d).value; }
static_assert(right_fold(Digit{1}, Digit{2}, Digit{3}) == 33);
static_assert(left_fold(Digit{1}, Digit{2}, Digit{3}) == 123);
