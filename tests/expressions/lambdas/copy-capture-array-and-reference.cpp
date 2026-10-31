// tidal-rules: LAM-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int arr[2] = {1, 2}; int x = 3; int& r = x; auto l = [arr, r] { return arr[1] * 10 + r; }; arr[1] = 9; x = 8; return l() == 23 ? 0 : 1; }
