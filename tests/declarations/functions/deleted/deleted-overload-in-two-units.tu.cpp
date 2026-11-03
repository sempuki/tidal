namespace io {
void take(double) = delete;
int take(int v) { return v * 2; }
}
int other() { return io::take(3); }
