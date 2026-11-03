// tidal-rules: CPY-19 CPY-27
// tidal-expect: accept
// tidal-note: Access plays no part in triviality. The private copy constructor of Sealed is trivial, so Sealed is trivially copyable. The union is a friend, so its copy constructor can use the private one, and UNI-3 does not delete it.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
union Box;
class Sealed {
  Sealed(const Sealed&) = default;
  friend union Box;
public:
  Sealed() = default;
  int value;
};
union Box { Sealed x; };
static_assert(!copies<Sealed> && copies<Box>);
