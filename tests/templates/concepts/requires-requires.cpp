// tidal-rules: CPT-9
// tidal-expect: accept
template <class T>
  requires requires (T t) { t + t; }
constexpr int twice(T t) { return t + t; }
struct Plain {};
template <class T> concept CanTwice = requires (T t) { twice(t); };
static_assert(CanTwice<int> && !CanTwice<Plain>);
