// Planted mistake: a call that picks a deleted overload.
void set(int) {}
void set(double) = delete;
int main() { set(2.5); return 0; }
