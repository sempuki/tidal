auto pick(bool b) { if (b) return 1; return 2.0; }
int main() { return static_cast<int>(pick(true)) - 1; }
