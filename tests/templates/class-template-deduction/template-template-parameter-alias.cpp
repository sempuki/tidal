// tidal-rules: CTAD-1
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-note: The alias A stands for a specialization of the template template parameter W. After substitution the alias names Box (Q-0275). Clang 18 has no deduction through an alias template (Q-0240).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
template <template <class> class W> struct Holder {
  template <class T> using A = W<T>;
  static constexpr bool check() { A a(1); return same<decltype(a), Box<int>>; }
};
static_assert(Holder<Box>::check());
