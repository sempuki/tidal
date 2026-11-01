// tidal-rules: CNS-5
// tidal-expect: accept
// tidal-note: For int, the left operand of the requires clause is not satisfied. The right operand would name int::tag, which is not valid, but it is not substituted, so the other overload is chosen.
template <class T> concept Tagged = requires { typename T::tag; };
template <class T> requires Tagged<T> && (sizeof(typename T::tag) < 4) constexpr int kind(T) { return 1; }
constexpr int kind(...) { return 2; }
struct Item { using tag = char; };
static_assert(kind(Item{}) == 1 && kind(0) == 2);
