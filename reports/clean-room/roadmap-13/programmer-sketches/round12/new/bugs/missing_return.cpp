// Planted bug: control flows off the end of a function that returns int.
int sign(int x) { if (x > 0) return 1; if (x < 0) return -1; }
int main() { volatile int z = 0; return sign(z); }
