int main() { int a[2] = {3, 4}; auto& [x, y] = a; x = 5; return a[0] + y == 9 ? 0 : 1; }
