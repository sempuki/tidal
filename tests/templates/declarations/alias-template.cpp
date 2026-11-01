// tidal-rules: TPL-28
// tidal-expect: accept
// tidal-note: A specialization of an alias template is the type that the alias gives, so a function can be declared with either spelling.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> using Pointer = T*;
template <class T> struct Box {};
template <class T> using BoxOf = Box<T>;
static_assert(same<Pointer<int>, int*>);
static_assert(same<BoxOf<char>, Box<char>>);
void take(int*);
void take(Pointer<int>);
