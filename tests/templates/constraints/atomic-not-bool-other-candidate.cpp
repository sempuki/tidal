// tidal-rules: CNS-9
// tidal-expect: reject
// tidal-error-clang: atomic constraint must be of type 'bool'
// tidal-error-gcc: not .bool.|does not have type .bool.
// tidal-error-edg: atomic constraint must have type bool
// tidal-note: The function with the ellipsis would be chosen, but the error in the constraint of the template stands.
struct Count { static constexpr int value = 1; };
template <class T> requires (T::value) int pick(T) { return 1; }
int pick(...) { return 2; }
int x = pick(Count{});
