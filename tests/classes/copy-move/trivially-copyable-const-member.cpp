// tidal-rules: CPY-19 CPY-7 UNI-3
// tidal-expect: accept
// tidal-note: A const member and a reference member delete the copy and move assignment operators (CPY-7). A deleted function is not eligible, and the constructors stay trivial, so CPY-19 counts both classes trivially copyable.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Fixed { const int value; };
static_assert(copies<Wrap<Fixed>> && moves<Wrap<Fixed>> && destroys<Wrap<Fixed>>);
static_assert(!copy_assigns<Fixed> && !move_assigns<Fixed>);
struct View { int& target; };
static_assert(copies<Wrap<View>> && moves<Wrap<View>> && destroys<Wrap<View>>);
static_assert(!copy_assigns<View> && !move_assigns<View>);
