// tidal-rules: DED-31
// tidal-expect: accept
// tidal-note: No value of T makes the result type the same as the needed type, so the result reaches it by a qualification adjustment. T takes the type at its own place in the needed type, with its qualifiers.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Two {
  template <class T> constexpr operator T**() const {
    static_assert(same<T, const volatile int>);
    return nullptr;
  }
};
struct Three {
  template <class T> constexpr operator T***() const {
    static_assert(same<T, const int>);
    return nullptr;
  }
};
const volatile int* const* two = Two{};
const int* const* const* three = Three{};
