consteval int scale(int x, int k = 10) { return x * k; }
constexpr int a = scale(3);
constexpr int b = scale(3, 2);
int main() { return a + b - 36; }
