int main() { int n = 0; for (const char* s : {"a", "b"}) n += s[0] == 'a'; return n - 1; }
