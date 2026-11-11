// [[assume]] with a condition that holds (C++23).
int half(int x) { [[assume(x % 2 == 0)]]; return x / 2; }
int main() { return half(8) - 4; }
