// tidal-rules: CPT-26 CPT-35
// tidal-expect: accept
// tidal-note: Outside a template, a requirement that fails can give false in two ways. A nested requirement on a concept-id is valid, and only its value is false. A concept with a parameter makes the requirement depend on a template parameter.
struct Label {};
struct Square { constexpr int area() const { return 4; } };
template <class T> concept Shape = requires (const T& s) { s.area(); };
static_assert(!requires { requires Shape<Label>; });
static_assert(requires { requires Shape<Square>; });
static_assert(!Shape<Label> && Shape<Square>);
template <class T, int N> requires (N > 0) struct Stack {};
template <class T, int N> concept Makes = requires { typename Stack<T, N>; };
static_assert(!Makes<int, 0> && Makes<int, 1>);
