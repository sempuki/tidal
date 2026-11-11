struct KV { int k, v; };
int main() { KV a[] = {{1, 2}, {3, 4}}; int s = 0; for (auto&& [k, v] : a) { v += k; s += v; } return s == 10 && a[1].v == 7 ? 0 : 1; }
