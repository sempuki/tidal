// tidal-rules: ENUM-9
// tidal-expect: run
namespace paint { enum class Color { red, green, blue }; }
namespace view { struct Theme { using enum paint::Color; }; }
using enum paint::Color;
int main() {
  if (green != paint::Color::green) return 1;
  if (view::Theme::blue != blue) return 2;
  {
    using enum ::paint::Color;
    if (red == blue) return 3;
  }
  return 0;
}
