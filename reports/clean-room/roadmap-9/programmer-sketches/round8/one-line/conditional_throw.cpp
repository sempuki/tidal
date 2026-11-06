int f(int x) { return x > 0 ? x : throw 1; }
int main() { try { f(-1); } catch (int) { return f(2) == 2 ? 0 : 1; } return 2; }
