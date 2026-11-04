// tidal-rules: UNI-7 UNI-4
// tidal-expect: accept
// tidal-note: A designator can name a member of the anonymous union directly, as in .level, and the union is initialized with that member active. A designator inside the union's own braces works too.
struct Reading {
  int id;
  union { int count; float level; };
  int unit;
};
constexpr Reading direct = {.id = 1, .level = 2.0f, .unit = 3};
constexpr Reading braced = {1, {.level = 4.0f}, 5};
static_assert(direct.level == 2.0f && direct.unit == 3);
static_assert(braced.level == 4.0f && braced.unit == 5);
