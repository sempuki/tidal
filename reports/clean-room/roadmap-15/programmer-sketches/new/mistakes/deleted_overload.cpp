int id(int x) { return x; }
int id(double) = delete;
int main() { return id(2.0); }
