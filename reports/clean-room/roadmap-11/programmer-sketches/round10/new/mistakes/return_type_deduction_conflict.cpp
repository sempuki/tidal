// Planted mistake: two returns deduce different types.
auto pick(bool b) { if (b) return 1; return 2.0; }
int main() { return pick(true); }
