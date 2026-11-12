struct G { int operator[](int r, int c) const { return r * c - 2; } };
int main() { G g; return g[1, 2]; }
