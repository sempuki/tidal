struct Shape { constexpr virtual int sides() const = 0; constexpr virtual ~Shape() = default; };
struct Tri : Shape { constexpr int sides() const override { return 3; } };
struct Quad : Shape { constexpr int sides() const override { return 4; } };
constexpr int total() { Tri t; Quad q; const Shape* s[] = {&t, &q, &t}; int n = 0; for (auto p : s) n += p->sides(); return n; }
static_assert(total() == 10);
int main() { return total() - 10; }
