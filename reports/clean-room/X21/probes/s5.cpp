int y = 2;
int x [[maybe_unused]] = 1;
int after(int v) { return v; }
int w = after(y);
int after_v = 3;
struct T { int after; };
int main() { return w == 2 ? 0 : 1; }
