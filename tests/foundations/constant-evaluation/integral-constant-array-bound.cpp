// tidal-rules: CE-11
// tidal-expect: accept
constexpr int n = 2 + 3;
int a[n * 2];
enum E { A = n };
static_assert(sizeof(a) == 10 * sizeof(int));
