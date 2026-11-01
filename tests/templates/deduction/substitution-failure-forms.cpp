// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: Each explicit argument makes a type that is not valid, so each template drops out and the template with the ellipsis wins. A plain function is not a candidate when the call gives explicit arguments.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One pointer(T*);
template <class T> Two pointer(...);
template <class T> One reference(T&);
template <class T> Two reference(...);
template <class T> One array(T (*)[1]);
template <class T> Two array(...);
template <class T> One member(int T::*);
template <class T> Two member(...);
template <class T> One result(T (*)());
template <class T> Two result(...);
template <class T> One empty(char (*)[sizeof(T) - sizeof(T)]);
template <class T> Two empty(...);
template <class T> One nested(typename T::type);
template <class T> Two nested(...);
static_assert(same<decltype(pointer<int&>(0)), Two>);
static_assert(same<decltype(reference<void>(0)), Two>);
static_assert(same<decltype(array<int&>(0)), Two>);
static_assert(same<decltype(member<int>(0)), Two>);
static_assert(same<decltype(result<int[2]>(0)), Two>);
static_assert(same<decltype(empty<int>(0)), Two>);
static_assert(same<decltype(nested<int>(0)), Two>);
