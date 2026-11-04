// tidal-rules: ICS-10
// tidal-expect: accept
// tidal-note: The object of Middle's conversion function and the parameter of Box's constructor accept only standard sequences.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Middle { operator int() const; };
struct Source { operator Middle() const; };
One take(int);
Two take(...);
static_assert(same<decltype(take(Source{})), Two>);
struct Number { operator int() const; };
struct Box { Box(int); };
One put(Box);
Two put(...);
static_assert(same<decltype(put(Number{})), Two>);
