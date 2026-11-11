// Designated initializers, with a member left to its default.
struct Options { int width = 80; int height = 24; bool color = false; };
int main() {
  Options o{.width = 100, .color = true};
  return (o.width == 100 && o.height == 24 && o.color) ? 0 : 1;
}
