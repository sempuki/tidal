// tidal-rules: UN-18
// tidal-expect: accept
struct Shape { virtual ~Shape() {} };
struct Circle : Shape {};
void check(Circle& c) {
  static_assert(noexcept(dynamic_cast<Shape&>(c)));
  static_assert(noexcept(dynamic_cast<const Shape&>(c)));
  static_assert(noexcept(dynamic_cast<Circle&>(c)));
}
int main() {}
