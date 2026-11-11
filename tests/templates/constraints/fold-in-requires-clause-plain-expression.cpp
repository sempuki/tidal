// tidal-rules: CNS-3 CNS-7
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer probe fold_in_requires_clause_plain_expression.cpp. A fold of comparisons in parentheses is a valid requires clause and gives one atomic constraint (CNS-3). The call with a char argument fails the constraint, so the function with an ellipsis is chosen.
template <class First, class... Rest>
  requires ((sizeof(First) == sizeof(Rest)) && ...)
constexpr int count_same(First, Rest...) { return 1 + sizeof...(Rest); }
constexpr int count_same(...) { return 0; }
int main() {
  if (count_same(7, 8, 9) != 3) return 1;
  return count_same(7, 'c') == 0 ? 0 : 2;
}
