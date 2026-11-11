// Planted mistake: two return statements deduce different types.
auto scale(bool big) { if (big) return 2.5; return 1; }
int main() { return static_cast<int>(scale(false)) - 1; }
