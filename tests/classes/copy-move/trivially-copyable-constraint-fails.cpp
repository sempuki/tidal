// tidal-rules: CPY-19 CNS-21 UNI-3
// tidal-expect: accept
// tidal-note: For K = 0 the constraint of the defaulted copy constructor fails, so only the user-provided copy constructor is eligible. Cell<0> is not trivially copyable, and the union's copy constructor is deleted. templates/constraints/eligible-copy.cpp checks the case where the constraint holds.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
template <int K> struct Cell {
  Cell() = default;
  Cell(const Cell&) requires (K > 0) = default;
  Cell(const Cell&) {}
};
static_assert(copies<Cell<0>> && !copies<Wrap<Cell<0>>>);
