// tidal-rules: CLS-9 CLS-2 LIST-8 LIST-3 LIST-9 LIST-4 INIT-9 INIT-1 CTOR-5 CTOR-4 CE-21
// tidal-expect: run
// tidal-note: Write suite W6, program default_members.cpp (reports/clean-room/W6/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A struct with default member values.
// Built from Tidal: CLS-2, CLS-9, LIST-3, LIST-4, LIST-8, LIST-9,
// INIT-9, CTOR-4, CTOR-5, CE-21.

struct Size {
  int w = 1;
  int h = 1;
};

struct Window {
  int width = 640;
  int height{480};
  double scale = 1.0;
  bool visible;                    // no default member initializer
  const char* title = "untitled";
  int area = width * height;       // reads earlier members (CLS-2)
  Size icon;                       // nested aggregate, its own defaults
};

// Every member gets a value: defaults, or zero for `visible` (LIST-8).
constexpr Window plain{};
static_assert(plain.width == 640 && plain.height == 480);
static_assert(!plain.visible);
static_assert(plain.area == 640 * 480);
static_assert(plain.icon.w == 1 && plain.icon.h == 1);

// A value in the list replaces the default; later defaults see it.
constexpr Window wide{
    .width = 800, .height = 600, .scale = 2.0, .visible = true, .icon{}};
static_assert(wide.area == 800 * 600);
static_assert(wide.title[0] == 'u');

// Designated values, in declaration order (LIST-5).
constexpr Window named{.width = 2, .height = 3, .visible = true, .icon{}};
static_assert(named.area == 6 && named.scale == 1.0);

// Inner braces for `icon`. Clang warns when they are elided (LIST-4).
constexpr Window full = {1, 2, 3.0, false, "t", 9, {4, 5}};
static_assert(full.area == 9 && full.icon.w == 4 && full.icon.h == 5);

// Parentheses: one value per member, no elision (LIST-3, LIST-9).
constexpr Window paren(10, 20, 1.5, true, "p", 7, Size{2, 3});
static_assert(paren.area == 7 && paren.icon.h == 3);

// A class with constructors is not an aggregate. A member initializer
// replaces the default member initializer (CTOR-5).
struct Counter {
  int start = 1;
  int step = 2;
  constexpr Counter() = default;
  constexpr explicit Counter(int s) : start(s) {}
  constexpr int at(int n) const { return start + n * step; }
};
static_assert(Counter().at(3) == 7);
static_assert(Counter(10).at(3) == 16);

// Every member has a default, so a const object needs no initializer
// (CTOR-4).
const Counter shared;

int main() {
  int result = 0;

  // Default initialization runs the implicit default constructor, which
  // uses the default member initializers (INIT-9). `visible` has no value,
  // so the program assigns it before any read (INIT-1).
  Window w;
  w.visible = false;
  if (w.width != 640 || w.area != 640 * 480 || w.visible) result |= 1;

  Window runtime{.width = 3, .visible = false, .icon = {}};
  if (runtime.area != 3 * 480 || runtime.visible) result |= 2;

  Counter c{5};
  if (c.at(1) != 7) result |= 4;
  if (shared.at(0) != 1) result |= 8;

  Size s{};
  Size t{7};
  if (s.w != 1 || t.w != 7 || t.h != 1) result |= 16;
  return result;
}
