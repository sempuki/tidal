// tidal-rules: PACK-1 DEFARG-8
// tidal-expect: accept
// tidal-note: The pattern of a function parameter pack can be any declarator: a reference, a pointer, or a pointer to function. A placeholder pattern makes a pack too, and parameters with defaults can come before a pack.
template <class... R> constexpr int functions(R (*... fp)()) { return sizeof...(fp); }
template <class... T> constexpr int references(const T&... x) { return sizeof...(x); }
template <class... T> constexpr int pointers(T*... p) { return sizeof...(p); }
constexpr int placeholders(auto... a) { return sizeof...(a); }
template <class... T> constexpr int after_default(int first = 1, T... rest) { return first + sizeof...(rest); }
int one();
char two();
int n = 0;
static_assert(functions(one, two) == 2);
static_assert(references(1, 'c', 2.0) == 3);
static_assert(pointers(&n, &n) == 2);
static_assert(placeholders() == 0 && placeholders(1, 'c') == 2);
static_assert(after_default() == 1 && after_default(5, 1, 2) == 7);
