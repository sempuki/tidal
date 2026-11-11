// Planted mistake: three names for a struct with two members.
struct Pair { int a, b; };
int main() { auto [x, y, z] = Pair{1, 2}; return x + y + z; }
