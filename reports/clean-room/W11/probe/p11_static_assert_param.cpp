consteval int check(int v) { static_assert(v > 0); return v; }
int main() { return check(1) - 1; }
