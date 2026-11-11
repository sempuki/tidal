struct Row { int v[3]; char tag; bool operator==(const Row&) const = default; };
constexpr Row a{{1, 2, 3}, 'x'}, b{{1, 2, 3}, 'x'}, c{{1, 2, 4}, 'x'};
static_assert(a == b && a != c);
int main() { return a == c; }
