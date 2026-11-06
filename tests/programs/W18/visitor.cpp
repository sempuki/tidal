// tidal-rules: UNI-4 UNI-5 UNI-7 LAM-15 PACK-14 PACK-21 CTAD-9 CTAD-27 OVL-27 RET-1
// tidal-expect: run
// tidal-note: Write suite W18, program visitor.cpp (reports/clean-room/W18/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A tagged union of three alternatives, like a Rust enum or a Swift enum
// with associated values, and a visitor made from lambdas.
struct Circle { int r; };
struct Rect { int w, h; };
struct Tri { int base, height; };

struct Shape {
  enum class Kind { circle, rect, tri } kind;
  union {
    Circle circle;
    Rect rect;
    Tri tri;
  };
  // A designator names the member of the anonymous union (UNI-7).
  static constexpr Shape make(Circle c) { return {.kind = Kind::circle, .circle = c}; }
  static constexpr Shape make(Rect r) { return {.kind = Kind::rect, .rect = r}; }
  static constexpr Shape make(Tri t) { return {.kind = Kind::tri, .tri = t}; }
};

// Like Rust's match: call the visitor with the active alternative.
template <class V>
constexpr auto visit(const Shape& s, V&& v) {
  switch (s.kind) {
    case Shape::Kind::circle: return v(s.circle);
    case Shape::Kind::rect: return v(s.rect);
    case Shape::Kind::tri: break;
  }
  // A switch with no default can complete (FN-7), so the last
  // alternative goes after it.
  return v(s.tri);
}

// An overload set made from several lambdas.
template <class... Fs>
struct Overload : Fs... {
  using Fs::operator()...;
};

constexpr int area_x2(const Shape& s) {
  return visit(s, Overload{
    [](const Circle& c) { return 6 * c.r * c.r; },   // about 2*pi*r*r
    [](const Rect& r) { return 2 * r.w * r.h; },
    []<class T>(const T& t) { return t.base * t.height; },
  });
}

// A generic visitor with a template head that names the alternative.
constexpr int tag_of(const Shape& s) {
  return visit(s, []<class T>(const T&) {
    if constexpr (requires(T t) { t.r; }) return 1;
    else if constexpr (requires(T t) { t.w; }) return 2;
    else return 3;
  });
}

constexpr int check() {
  Shape a = Shape::make(Circle{2});
  Shape b = Shape::make(Rect{3, 4});
  Shape c = Shape::make(Tri{5, 6});
  if (area_x2(a) != 24) return 1;
  if (area_x2(b) != 24) return 2;
  if (area_x2(c) != 30) return 3;
  if (tag_of(a) != 1 || tag_of(b) != 2 || tag_of(c) != 3) return 4;
  // Change the alternative by assignment (UNI-5).
  b.kind = Shape::Kind::circle;
  b.circle = Circle{1};
  if (area_x2(b) != 6) return 5;
  return 0;
}

static_assert(check() == 0);

int main() { return check(); }
