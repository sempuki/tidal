// tidal-rules: TSP-1
// tidal-expect: accept
// tidal-note: The explicit specialization Box<bool> has the member flip and lacks the member value of the template.
template <class T> struct Box { T value{}; };
template <> struct Box<bool> { constexpr int flip() const { return 1; } };
template <class B> concept HasValue = requires (B b) { b.value; };
static_assert(HasValue<Box<int>> && !HasValue<Box<bool>>);
static_assert(Box<bool>{}.flip() == 1);
