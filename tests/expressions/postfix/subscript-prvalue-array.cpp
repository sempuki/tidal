// tidal-rules: PFX-3
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0034. EDG does not give an xvalue for a subscript of an array member of a prvalue; Clang and GCC do.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Holder { int v[2]; };
Holder make();
static_assert(same<decltype((make().v[0])), int&&>);
