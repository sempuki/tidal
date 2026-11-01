// tidal-rules: CPT-19
// tidal-expect: accept
// tidal-note: One is satisfied only by a pack of one type. On a pack, the type constraint checks One<T> for each element on its own.
template <class... T> concept One = sizeof...(T) == 1;
template <One... T> struct Many {};
template <One T> struct Single {};
template <class... T> concept MakesMany = requires { typename Many<T...>; };
static_assert(MakesMany<int, char, long>);
Single<int> single;
