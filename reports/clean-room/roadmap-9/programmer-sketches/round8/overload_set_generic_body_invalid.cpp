struct Circle { int r; };
struct Rect { int base; };

template <class... F> struct overloaded : F... { using F::operator()...; };

int main() {
  overloaded area{
    [](const Circle& c) { return 3 * c.r * c.r; },
    [](const auto& s) { return s.base; },
  };
  return area(Circle{1}) + area(Rect{2}) == 5 ? 0 : 1;
}
