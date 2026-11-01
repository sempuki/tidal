// tidal-rules: DED-14
// tidal-expect: accept
// tidal-note: The substituted parameter types differ from the arguments by added const and by a dropped noexcept.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> deep(const T* const* p);
template <class R> Tag<R> call(R (*)());
template <class T, class C> Tag<T> field(const T C::*);
struct Owner { int value; };
int** pp = nullptr;
int give() noexcept;
static_assert(same<decltype(deep(pp)), Tag<int>>);
static_assert(same<decltype(call(give)), Tag<int>>);
static_assert(same<decltype(field(&Owner::value)), Tag<int>>);
