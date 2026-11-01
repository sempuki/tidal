// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: The pattern C<T> matches any specialization of a one-parameter class template.
template <class T> struct Box {};
template <class T> struct List {};
template <class T> struct Inner { static constexpr int depth = 0; };
template <template <class> class C, class T> struct Inner<C<T>> { static constexpr int depth = 1 + Inner<T>::depth; };
static_assert(Inner<int>::depth == 0);
static_assert(Inner<Box<int>>::depth == 1);
static_assert(Inner<List<Box<int>>>::depth == 2);
