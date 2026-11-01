// tidal-rules: TSP-12
// tidal-expect: reject
// tidal-error-clang: partial specialization cannot be declared as a friend
// tidal-error-gcc: must appear at namespace scope
// tidal-error-edg: a friend declaration may not declare a partial
template <class T> struct Box {};
struct Owner {
  template <class T> friend struct Box<T*>;
};
