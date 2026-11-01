// tidal-rules: CPT-1
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> concept Wide = sizeof(T) >= 4;
static_assert(Wide<int>);
static_assert(!Wide<char>);
static_assert(same<decltype(Wide<int>), bool>);
template <bool B> struct Flag { static constexpr bool value = B; };
static_assert(Flag<Wide<double>>::value);
constexpr int pick() {
  if constexpr (Wide<char>) return 1;
  else return 2;
}
static_assert(pick() == 2);
constexpr bool stored = Wide<int>;
static_assert(stored);
