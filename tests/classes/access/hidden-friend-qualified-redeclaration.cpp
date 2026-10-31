// tidal-rules: ACC-17
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-note: Q-0131
// tidal-error-gcc: geo::make_seven.*has not been declared within .geo
// tidal-error-edg: function .geo::make_seven. may not be redeclared in the current scope
namespace geo {
  class Point { friend int make_seven() { return 7; } };
}
int geo::make_seven();
