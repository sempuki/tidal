// tidal-rules: TARG-11
// tidal-expect: accept
// tidal-note: In an explicit and a partial specialization, the injected class name alone names that specialization.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { using self = Box; };
template <> struct Box<int> { using self = Box; };
template <class T> struct Box<T*> { using self = Box; };
static_assert(same<Box<int>::self, Box<int>>);
static_assert(same<Box<char*>::self, Box<char*>>);
