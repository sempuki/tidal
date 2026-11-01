// tidal-rules: TARG-1
// tidal-expect: accept
// tidal-note: A qualified name that finds a template starts an argument list too.
namespace lib {
  template <class T> struct Box { static constexpr int tag = 1; };
  template <class T> constexpr int width = sizeof(T);
}
static_assert(lib::Box<int>::tag == 1);
static_assert(lib::width<short> == sizeof(short));
