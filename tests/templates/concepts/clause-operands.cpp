// tidal-rules: CPT-9
// tidal-expect: accept
template <class T> constexpr bool big() { return sizeof(T) > 1; }
template <class T> constexpr bool small = sizeof(T) == 1;
template <class T> concept Any = true;
template <class T>
  requires (big<T>()) || small<T> || true && Any<T> && (sizeof(T) < 100)
constexpr int pick(T) { return 1; }
static_assert(pick(1) == 1 && pick('a') == 1);
