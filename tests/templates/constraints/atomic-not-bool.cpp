// tidal-rules: CNS-9
// tidal-expect: reject
// tidal-error-clang: atomic constraint must be of type 'bool'
// tidal-error-gcc: not .bool.|does not have type .bool.
// tidal-error-edg: atomic constraint must have type bool
struct Count { static constexpr int value = 1; };
template <class T> concept Valued = T::value;
static_assert(Valued<Count>);
