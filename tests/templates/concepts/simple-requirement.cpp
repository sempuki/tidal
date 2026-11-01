// tidal-rules: CPT-31
// tidal-expect: accept
struct Counter { int value; void bump(); };
template <class T> concept Bumps = requires (T t, const T c) { t.bump(); c.value; t.value = 1; };
template <class T> concept Assigns = requires (const T c) { c.value = 1; };
static_assert(Bumps<Counter> && !Bumps<int> && !Assigns<Counter>);
