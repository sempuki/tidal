// tidal-rules: TARG-11
// tidal-expect: accept
// tidal-note: The injected name stands for the template where a template template argument is needed.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <template <class> class C> struct Use { using type = C<long>; };
template <class T> struct Node {
  using made = typename Use<Node>::type;
};
static_assert(same<Node<int>::made, Node<long>>);
