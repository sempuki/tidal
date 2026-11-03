// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-note: A namespace declares a class and then a variable with the same name.
namespace geo {
  struct extent { int width; };
  int extent = 3;
}
