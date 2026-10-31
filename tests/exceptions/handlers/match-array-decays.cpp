// tidal-rules: TRY-2
// tidal-expect: run
int arr[3] = {1, 2, 3};
int main() { try { throw arr; } catch (int* p) { return p[2] == 3 ? 0 : 1; } return 2; }
