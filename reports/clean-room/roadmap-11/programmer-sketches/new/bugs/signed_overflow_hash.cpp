// A Java-style hash in int that overflows.
int hash(const char* s) { int h = 0; for (; *s; ++s) h = h * 31 + *s; return h; }
int main() { return hash("overflowing key") == 0; }
