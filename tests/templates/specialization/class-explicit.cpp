// tidal-rules: TSP-1
// tidal-expect: accept
// tidal-note: The explicit specialization has other members than the template.
template <class T> struct Traits { static constexpr int size = sizeof(T); };
template <> struct Traits<void> { static constexpr int size = 0; using marker = int; };
static_assert(Traits<char>::size == 1);
static_assert(Traits<void>::size == 0);
static_assert(sizeof(Traits<void>::marker) == sizeof(int));
