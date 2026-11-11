// Two increments of one variable with no order between them.
int main() { int i = 1; int v = i++ + i++; return v == 3 ? 0 : 1; }
