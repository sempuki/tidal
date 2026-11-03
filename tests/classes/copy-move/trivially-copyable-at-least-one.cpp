// tidal-rules: CPY-19 UNI-3
// tidal-expect: accept
// tidal-note: Only has a trivial move constructor and three deleted copy and move functions, so CPY-19 counts it trivially copyable. None has all four deleted and no eligible copy or move function, so CPY-19 does not count it (Q-0343). The union wrapper shows which functions are trivial.
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Only {
  Only() = default;
  Only(Only&&) = default;
  Only(const Only&) = delete;
  Only& operator=(const Only&) = delete;
  Only& operator=(Only&&) = delete;
};
static_assert(moves<Wrap<Only>> && destroys<Wrap<Only>>);
static_assert(!copies<Wrap<Only>> && !copy_assigns<Wrap<Only>> && !move_assigns<Wrap<Only>>);
struct None {
  None() = default;
  None(const None&) = delete;
  None(None&&) = delete;
  None& operator=(const None&) = delete;
  None& operator=(None&&) = delete;
};
static_assert(!copies<None> && !moves<None> && !copy_assigns<None> && !move_assigns<None>);
static_assert(destroys<Wrap<None>>);
