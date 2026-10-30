// tidal-rules: CX-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
int arr[3];
constexpr int* end = arr + 3;
void f();
constexpr void (*fp)() = f;
