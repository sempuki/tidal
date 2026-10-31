// tidal-rules: CE-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
enum E { a, b };
const E e = b;
int arr[e + 1];
static_assert(e == b);
