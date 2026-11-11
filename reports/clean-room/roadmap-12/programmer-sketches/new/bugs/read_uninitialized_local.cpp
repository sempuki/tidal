// Planted bug: a local read before any value is stored.
int pick(bool b) { int r; if (b) r = 1; return r; }
int main() { volatile bool f = false; return pick(f); }
