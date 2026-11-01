// tidal-rules: CPT-33
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: t.missing() is not valid for int, so the requirement fails before Trap<int> is formed. Clang forms Trap<int> and its static assertion fails (Q-0287).
template <class T> struct Trap { static_assert(sizeof(T) == 0); using type = T; };
template <class T, class U> concept Both = sizeof(T) > 0 && sizeof(U) > 0;
template <class T> concept Missing = requires (T t) {
  { t.missing() } -> Both<typename Trap<T>::type>;
};
static_assert(!Missing<int>);
