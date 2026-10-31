// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
enum class E { one = 1 };
int arr[4];
int* p = arr + E::one;
