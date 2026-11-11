// Structured bindings to an array and by reference to a struct.
struct Pair { int a; double b; };
int main() { int xs[3] = {1, 2, 3}; auto [x, y, z] = xs; Pair p{4, 0.5}; auto& [i, d] = p; i = 7; return x + y + z + p.a == 13 && d == 0.5 ? 0 : 1; }
