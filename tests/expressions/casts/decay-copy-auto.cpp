// tidal-rules: CAST-13
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int v = 3; auto c = auto(v); c = 9; int arr[2] = {1, 2}; auto p = auto{arr}; return v == 3 && p == arr ? 0 : 1; }
