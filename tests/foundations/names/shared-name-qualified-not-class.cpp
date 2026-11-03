// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: no type named .point. in namespace .geo
// tidal-error-gcc: point. in namespace .geo. does not name a type
// tidal-error-edg: variable .geo::point. is not a type name
// tidal-note: geo::point finds the variable, so the qualified name alone cannot name the class.
namespace geo { struct point { int x; }; int point = 3; }
geo::point origin;
