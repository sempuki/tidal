// tidal-rules: SB-1
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int arr[2] = {3, 4}; auto [a, b] = arr; arr[0] = 9; return a == 3 && b == 4 ? 0 : 1; }
