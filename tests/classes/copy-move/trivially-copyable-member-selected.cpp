// tidal-rules: CPY-27 CPY-19 UNI-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Q-0339. The implicit copy constructor of Holder copies its member with the trivial constructor of Twin for a const source, so the copy constructor of Holder is trivial, and Holder is trivially copyable. The union can be copied. GCC and EDG treat the copy constructor of Holder as not trivial and delete the union's copy constructor.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Twin { Twin() = default; Twin(const Twin&) = default; Twin(Twin&) {} };
struct Holder { Twin part; };
void copy(const Wrap<Holder>& from) { new Wrap<Holder>(from); }
