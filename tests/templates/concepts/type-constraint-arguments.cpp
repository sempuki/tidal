// tidal-rules: CPT-17
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class U> concept SameAs = same<T, U>;
template <SameAs<int> T> constexpr int one = 1;
template <class T> constexpr bool has_one = requires { one<T>; };
static_assert(has_one<int> && !has_one<char>);
