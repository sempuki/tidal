// tidal-rules: CNS-16
// tidal-expect: accept
template <class T> struct Box {
  Box() requires (sizeof(T) > 100) {}
  Box(int) {}
};
template <class T> concept Makes = requires { T(); };
static_assert(!Makes<Box<int>>);
