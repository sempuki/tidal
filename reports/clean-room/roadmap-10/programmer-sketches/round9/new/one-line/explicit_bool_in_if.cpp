struct Handle { int fd = -1; explicit operator bool() const { return fd >= 0; } };
int main() { Handle h{3}; Handle n; if (h && !n) return 0; return 1; }
