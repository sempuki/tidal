// tidal-rules: CPY-28 STG-12
// tidal-expect: run
// tidal-note: Arrays, scalars, and const objects of trivially copyable types keep their values through a byte copy.
struct Point { int x; int y; };
enum Color { red, green };
template <class T> void copy_bytes(const T& from, T& to) {
  const unsigned char* src = reinterpret_cast<const unsigned char*>(&from);
  unsigned char* dst = reinterpret_cast<unsigned char*>(&to);
  for (unsigned i = 0; i < sizeof(T); ++i) dst[i] = src[i];
}
int main() {
  const Point source[2] = {{1, 2}, {3, 4}};
  Point points[2] = {};
  copy_bytes(source, points);
  int Point::* member = &Point::y;
  int Point::* member_copy = nullptr;
  copy_bytes(member, member_copy);
  Color color = green, color_copy = red;
  copy_bytes(color, color_copy);
  const Point* pointer = &source[1];
  const Point* pointer_copy = nullptr;
  copy_bytes(pointer, pointer_copy);
  return points[1].y == 4 && points[0].x == 1 && member_copy == &Point::y && color_copy == green && pointer_copy->x == 3 ? 0 : 1;
}
