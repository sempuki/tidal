// tidal-rules: TARG-19
// tidal-expect: accept
// tidal-note: Pointer arguments are the same when they point to the same object.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
int a = 0;
int b = 0;
int& alias = a;
template <int* P> struct Ptr {};
template <int& R> struct Ref {};
static_assert(same<Ptr<&a>, Ptr<&a>>);
static_assert(!same<Ptr<&a>, Ptr<&b>>);
static_assert(same<Ref<a>, Ref<alias>>);
