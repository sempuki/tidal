// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-error-clang: explicit object parameter cannot appear in a static function
// tidal-error-gcc: explicit object member function cannot be .static
// tidal-error-edg: cannot have an explicit .this. parameter
// tidal-note: Gap review 2026-10-30.
struct S {
  static int get(this S self);
};
