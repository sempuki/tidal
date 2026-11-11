// A virtual call in constant evaluation (C++20).
struct Shape { constexpr virtual int sides() const { return 0; } constexpr virtual ~Shape() = default; };
struct Tri : Shape { constexpr int sides() const override { return 3; } };
constexpr int count(const Shape& s) { return s.sides(); }
static_assert(count(Tri{}) == 3);
int main() { Tri t; return count(t) - 3; }
