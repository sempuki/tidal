// tidal-rules: INST-27
// tidal-expect: accept
template <class T> constexpr int size_of = sizeof(T);
template <class T> struct Box { static const int count = 2; static constexpr int size = sizeof(T); };
extern template const int size_of<int>;
extern template struct Box<int>;
static_assert(size_of<int> == sizeof(int));
static_assert(Box<int>::count == 2 && Box<int>::size == sizeof(int));
