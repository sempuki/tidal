// tidal-rules: NAM-16 TYN-1
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction next_alias_of_class_qualifier_false_reject.cpp and probe next_alias_of_specialization_qualifier.cpp, from the rewrite's runs. Alias names the class Limits and Boxed names the specialization Box<char> (TYN-1). Each is a complete class, so each is valid as the qualifier before :: (NAM-16).
struct Limits { static constexpr int most = 9; using count = int; };
template <class T> struct Box { static constexpr int size = sizeof(T); };
using Alias = Limits;
using Boxed = Box<char>;
static_assert(Alias::most == 9 && Boxed::size == 1);
int main() {
  Alias::count n = Alias::most;
  return n == 9 && Boxed::size == 1 ? 0 : 1;
}
