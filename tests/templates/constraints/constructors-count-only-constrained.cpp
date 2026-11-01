// tidal-rules: CNS-16
// tidal-expect: accept
// tidal-note: The only constructor of Box is a default constructor whose constraint fails for Box<int>. The class still has that constructor, so no implicit one is added and Box<int> cannot be built from nothing.
template <class T> struct Box {
  Box() requires (sizeof(T) > 100) {}
};
struct Huge { char bytes[128]; };
template <class T> concept Makes = requires { T(); };
static_assert(!Makes<Box<int>>);
static_assert(Makes<Box<Huge>>);
