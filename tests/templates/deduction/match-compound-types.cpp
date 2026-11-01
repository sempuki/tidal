// tidal-rules: DED-10
// tidal-expect: accept
// tidal-note: Each form of the parameter type is matched part by part against the argument type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> struct Box {};
template <class T, class U> struct Pair {};
struct Owner { int field; char method(long); };
template <class T> Tag<T> ptrptr(T**);
template <class T, class C> Tag<void(T, C)> member(T C::*);
template <class R, class A> Tag<R(A)> fnptr(R (*)(A));
template <class T> Tag<T> boxed(Box<T>);
template <class T> Tag<T> boxptr(const Box<T*>&);
template <class T, class U> Tag<void(T, U)> paired(Pair<T, Box<U>>);
template <template <class> class B, class T> Tag<B<T>> any(B<T>);
template <class R, class... A> Tag<R(A...)> whole(R (&)(A...));
int** pp = nullptr;
char twice(char);
long both(int, char);
static_assert(same<decltype(ptrptr(pp)), Tag<int>>);
static_assert(same<decltype(member(&Owner::field)), Tag<void(int, Owner)>>);
static_assert(same<decltype(member(&Owner::method)), Tag<void(char(long), Owner)>>);
static_assert(same<decltype(fnptr(&twice)), Tag<char(char)>>);
static_assert(same<decltype(boxed(Box<int>{})), Tag<int>>);
static_assert(same<decltype(boxptr(Box<char*>{})), Tag<char>>);
static_assert(same<decltype(paired(Pair<int, Box<char>>{})), Tag<void(int, char)>>);
static_assert(same<decltype(any(Box<long>{})), Tag<Box<long>>>);
static_assert(same<decltype(whole(both)), Tag<long(int, char)>>);
