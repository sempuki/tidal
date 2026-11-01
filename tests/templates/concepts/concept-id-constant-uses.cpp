// tidal-rules: CPT-1
// tidal-expect: accept
// tidal-note: A concept-id stands as an array bound, a case label, and a noexcept operand. Inside a template its value follows the template argument.
template <class T> concept Wide = sizeof(T) >= 4;
char cells[Wide<int> + 1];
static_assert(sizeof cells == 2);
constexpr int pick(int v) {
  switch (v) {
    case Wide<char>: return 10;
    case Wide<int>: return 20;
  }
  return 0;
}
static_assert(pick(0) == 10 && pick(1) == 20);
void maybe() noexcept(Wide<int>);
static_assert(noexcept(maybe()));
template <class T> constexpr int width = Wide<T> ? 4 : 1;
static_assert(width<long> == 4 && width<char> == 1);
