int s;
struct R { operator int&() { return s; } };
int main() { R r; int i = 0; (true ? r : i) = 1; return s - 1; }
