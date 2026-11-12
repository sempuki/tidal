struct Point { int x = 0, y = 0; };
struct Rect { Point lo, hi; int tag = 7; };
constexpr Rect r{.lo = {.x = 1}, .hi = {.x = 4, .y = 5}};
static_assert(r.lo.y == 0 && r.hi.y == 5 && r.tag == 7);
int main() { return r.hi.x - r.lo.x - 3; }
