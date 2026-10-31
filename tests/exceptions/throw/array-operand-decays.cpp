// tidal-rules: THR-2
// tidal-expect: run
int main() { int arr[3] = {7, 8, 9}; try { throw arr; } catch (int* p) { return p[2] == 9 ? 0 : 1; } return 2; }
