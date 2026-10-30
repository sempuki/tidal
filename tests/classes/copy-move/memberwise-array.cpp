// tidal-rules: CPY-8
// tidal-expect: run
struct S { int arr[3] = {1, 2, 3}; };
int main() { S a; a.arr[2] = 9; S b(a); S c; c = a; return b.arr[2] == 9 && c.arr[2] == 9 && b.arr[0] == 1 ? 0 : 1; }
