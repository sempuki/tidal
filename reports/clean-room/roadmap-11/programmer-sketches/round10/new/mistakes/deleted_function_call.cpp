// Planted mistake: a call that picks a deleted overload.
void take(int) {}
void take(double) = delete;
int main() { take(1.5); return 0; }
