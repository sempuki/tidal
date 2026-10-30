// tidal-rules: PFX-2
// tidal-expect: reject
enum class Index { First };
int a[2];
int f() { return a[Index::First]; }
