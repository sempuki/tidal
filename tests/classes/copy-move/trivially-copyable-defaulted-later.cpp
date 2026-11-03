// tidal-rules: CPY-19 CPY-27 UNI-3
// tidal-expect: accept
// tidal-note: A copy constructor defaulted after its first declaration is user-provided, so it is not trivial and the class is not trivially copyable. The same function defaulted in the class is trivial.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Later { Later() = default; Later(const Later&); int value; };
Later::Later(const Later&) = default;
struct InClass { InClass() = default; InClass(const InClass&) = default; int value; };
static_assert(copies<Later> && !copies<Wrap<Later>>);
static_assert(copies<Wrap<InClass>>);
