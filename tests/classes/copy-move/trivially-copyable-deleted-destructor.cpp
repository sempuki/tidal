// tidal-rules: CPY-19 CPY-7 UNI-3 DTOR-3
// tidal-expect: accept
// tidal-note: The destructor of Pinned is deleted, so CPY-19 does not count Pinned trivially copyable, although its copy and move functions are trivial (Q-0343). The union wrapper's destructor is deleted, because its variant member has a deleted destructor (DTOR-3), and so are its copy and move constructors (CPY-7). Its assignments stay usable.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Pinned { int value; ~Pinned() = delete; };
static_assert(!destroys<Wrap<Pinned>> && !copies<Wrap<Pinned>> && !moves<Wrap<Pinned>>);
static_assert(copy_assigns<Wrap<Pinned>> && move_assigns<Wrap<Pinned>>);
