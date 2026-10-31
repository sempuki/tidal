// tidal-rules: CVF-2
// tidal-expect: run
struct Owner { int* p; operator int*() const { return p; } };
int main() { Owner o{new int(3)}; delete o; return 0; }
