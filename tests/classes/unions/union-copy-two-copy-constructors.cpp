// tidal-rules: UNI-3 CPY-27
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Q-0339. The union copies its member from a const source, with the trivial copy constructor of Twin, so UNI-3 does not delete the union's copy constructor. GCC and EDG delete it because Twin has another copy constructor that is not trivial.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Twin { Twin() = default; Twin(const Twin&) = default; Twin(Twin&) {} };
void copy(const Wrap<Twin>& from) { new Wrap<Twin>(from); }
