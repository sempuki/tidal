// tidal-rules: CPT-27
// tidal-expect: reject
// tidal-error-clang: deleted function
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Write report W5. The requirement does not depend on a template parameter, so the call of the deleted function makes the program not valid.
int twice(int v) { return 2 * v; }
int twice(double) = delete;
static_assert(!requires { twice(1.5); });
