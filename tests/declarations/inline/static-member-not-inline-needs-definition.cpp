// tidal-rules: INL-4
// tidal-expect: link-fail
struct S { static int count; };
int main() { return S::count; }
