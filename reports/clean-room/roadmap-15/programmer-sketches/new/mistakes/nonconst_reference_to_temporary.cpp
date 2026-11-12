int bump(int& x) { return ++x; }
int main() { return bump(41) - 42; }
