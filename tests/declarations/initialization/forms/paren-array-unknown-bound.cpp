// tidal-rules: INIT-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int arr[](1, 2, 3); return sizeof(arr) / sizeof(arr[0]) == 3 ? 0 : 1; }
