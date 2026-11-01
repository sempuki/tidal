// tidal-rules: TSP-9
// tidal-expect: accept
// tidal-note: The partial specialization is declared twice, then defined with another parameter name.
template <class T> struct Box { static constexpr int kind = 0; };
template <class T> struct Box<T*>;
template <class T> struct Box<T*>;
template <class U> struct Box<U*> { static constexpr int kind = 1; };
static_assert(Box<int*>::kind == 1);
