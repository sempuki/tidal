// tidal-rules: DED-31
// tidal-expect: accept
// tidal-note: T** with T = const int is not const int* const*, but a qualification conversion reaches it. Quiet<int> drops its noexcept to reach int (*)().
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct Deep {
  template <class T> constexpr operator T**() const {
    static_assert(same<T, const int>);
    return nullptr;
  }
};
template <class R> using Quiet = R (*)() noexcept;
struct Call { template <class R> operator Quiet<R>() const { return nullptr; } };
const int* const* pp = Deep{};
int (*fp)() = Call{};
