// tidal-rules: CPT-32
// tidal-expect: accept
// tidal-note: Box<char> fails the constraint of Box, and Pair<int> fails the substitution into its default argument.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box {};
template <class T, class U = typename T::type> struct Pair {};
template <class T> concept HasBox = requires { typename Box<T>; };
template <class T> concept HasPair = requires { typename Pair<T>; };
static_assert(HasBox<int> && !HasBox<char>);
static_assert(!HasPair<int>);
