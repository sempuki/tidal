// tidal-rules: CPY-19 CPY-27 STG-12
// tidal-expect: run
// tidal-note: A volatile int member leaves each copy and move function trivial, so the class is trivially copyable. A byte copy then gives an equal object. The copy uses volatile byte glvalues, because a read of the volatile member through a glvalue that is not volatile is undefined (VOL-6, Q-0359).
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool copy_assigns = requires (T& a, const T& b) { a = b; };
template <class T> constexpr bool move_assigns = requires (T& a, T& b) { a = static_cast<T&&>(b); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
template <class T> union Wrap { T x; };  // each special member is deleted if the one of T is not trivial (UNI-3)
struct Port { volatile int status; int count; };
static_assert(copies<Wrap<Port>> && moves<Wrap<Port>> && copy_assigns<Wrap<Port>> && move_assigns<Wrap<Port>> && destroys<Wrap<Port>>);
int main() {
  Port a{7, 9}, b{0, 0};
  volatile unsigned char* to = reinterpret_cast<volatile unsigned char*>(&b);
  const volatile unsigned char* from = reinterpret_cast<const volatile unsigned char*>(&a);
  for (unsigned i = 0; i < sizeof(Port); ++i) to[i] = from[i];
  return b.status == 7 && b.count == 9 ? 0 : 1;
}
