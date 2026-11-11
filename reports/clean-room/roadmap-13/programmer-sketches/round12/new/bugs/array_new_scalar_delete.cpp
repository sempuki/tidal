// Planted bug: an array from new[] freed with delete.
struct Buf { int* p = new int[4]{}; ~Buf() { delete p; } };
int main() { Buf b; b.p[3] = 1; return 0; }
