// Planted mistake: a goto that skips a variable with an initializer.
int main() { int r = 0; goto done; { } int extra = 5; done: r = extra; return r - 5; }
