// tidal-rules: CNS-3 CNS-7
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction fold_in_requires_clause.cpp (from the tries of units.cpp). A fold of concept-ids in parentheses is a valid requires clause and gives one atomic constraint (CNS-3). The constraint holds for a pack of small types and fails when one type is large.
template <class T> concept Narrow = sizeof(T) <= 4;
template <class... Ts>
  requires (Narrow<Ts> && ...)
constexpr int count_narrow() { return sizeof...(Ts); }
template <class... Ts> concept Countable = requires { count_narrow<Ts...>(); };
struct Wide { char bytes[16]; };
int main() {
  if (!Countable<int, char> || Countable<int, Wide>) return 1;
  return count_narrow<int, char, short>() == 3 ? 0 : 2;
}
