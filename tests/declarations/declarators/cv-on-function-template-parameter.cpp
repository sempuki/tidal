// tidal-rules: CV-6
// tidal-expect: accept
// tidal-note: I5-3. const T with T a function type is that function type, as through an alias. const T with T an array type is an array of const elements.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Add { using type = const T; };
static_assert(same<Add<int(char)>::type, int(char)>);
static_assert(same<Add<int[2]>::type, const int[2]>);
