// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-note: A class template and an alias template, each with a parameter list like the parameter's.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct Box {};
template <class T> using Pointer = T*;
template <template <class> class C> struct Apply { using type = C<int>; };
static_assert(same<Apply<Box>::type, Box<int>>);
static_assert(same<Apply<Pointer>::type, int*>);
