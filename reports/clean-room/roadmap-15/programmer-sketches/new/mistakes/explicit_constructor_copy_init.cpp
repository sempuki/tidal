struct Meters { explicit Meters(int v) : v(v) {} int v; };
int length(Meters m) { return m.v; }
int main() { return length(3) - 3; }
