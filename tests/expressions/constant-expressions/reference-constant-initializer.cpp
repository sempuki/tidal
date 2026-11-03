// tidal-rules: CE-3
// tidal-expect: accept
// tidal-note: alias is not constexpr, but its initializer is a constant expression. Constant evaluation can use alias to name a, also in a template argument.
int a = 0;
int& alias = a;
constexpr int* p = &alias;
template <int& R> struct Ref {};
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
static_assert(p == &a);
static_assert(same<Ref<alias>, Ref<a>>);
