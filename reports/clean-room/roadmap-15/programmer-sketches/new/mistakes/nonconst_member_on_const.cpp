struct Box { int v = 0; int& get() { return v; } };
int read(const Box& b) { return b.get(); }
int main() { return read(Box{}); }
