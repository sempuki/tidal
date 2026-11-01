// tidal-rules: INST-27
// tidal-expect: accept
template <class T> constexpr int get(T) { return 1; }
template <class T> struct Box { int value = 2; };
extern template int get(int);
extern template struct Box<int>;
static_assert(get(0) == 1);
static_assert(sizeof(Box<int>) == sizeof(int));
