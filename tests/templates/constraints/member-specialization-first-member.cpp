// tidal-rules: CNS-37
// tidal-expect: accept
// tidal-note: For Box<Item>, the clauses of both members are valid. The specialization's clause is equivalent to that of the first member after substitution, so it specializes the first member.
struct Item { using type = int; };
template <class T> struct Box {
  template <class U> static constexpr int f(U) requires (sizeof(T) == 1) { return 1; }
  template <class U> static constexpr int f(U) requires (sizeof(T) > 1) { return 2; }
};
template <> template <class U> constexpr int Box<Item>::f(U) requires (sizeof(Item) == 1) { return 3; }
static_assert(Box<Item>::f(1) == 3);
static_assert(Box<long>::f(1) == 2);
