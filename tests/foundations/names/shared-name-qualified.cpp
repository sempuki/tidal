// tidal-rules: NAM-40
// tidal-expect: run
// tidal-note: N::name and C::name find the variable. struct N::name and the qualifier N::name:: find the class. A scoped enumeration stays reachable through its qualifier.
namespace geo {
  struct point { static const int dims = 2; int x = 4; };
  int point = 3;
}
struct grid {
  struct cell { int v = 5; };
  static const int cell = 6;
};
enum class mode { on = 7 };
int mode = 8;
int main() {
  struct geo::point p;
  struct grid::cell c;
  bool ok = geo::point == 3 && geo::point::dims == 2 && p.x == 4 &&
            grid::cell == 6 && c.v == 5 &&
            static_cast<int>(mode::on) == 7 && mode == 8;
  return ok ? 0 : 1;
}
