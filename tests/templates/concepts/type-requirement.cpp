// tidal-rules: CPT-32
// tidal-expect: accept
struct Tagged { using tag = int; template <class> struct Inner {}; };
template <class T> struct Box {};
template <class T> using Ptr = T*;
template <class T> concept Shapes = requires {
  typename T::tag;
  typename T::template Inner<int>;
  typename Box<T>;
  typename Ptr<T>;
};
static_assert(Shapes<Tagged> && !Shapes<int>);
template <class T> concept Pointable = requires { typename Ptr<T>; };
static_assert(Pointable<int> && !Pointable<int&>);
