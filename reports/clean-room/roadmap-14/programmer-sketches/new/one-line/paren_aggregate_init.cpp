struct P { int x; long y; };
struct Q { P p; int z; };
constexpr P a(1, 2);
constexpr Q b(P(3, 4), 5);
static_assert(a.y == 2 && b.p.x == 3 && b.z == 5);
int main() { P* p = new P(6, 7); int r = p->x + static_cast<int>(p->y); delete p; return r - 13; }
