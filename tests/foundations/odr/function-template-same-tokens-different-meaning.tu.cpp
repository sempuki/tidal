static const int factor = 3;
template <class T> T scaled(T v) { return v * factor; }
int scaled_elsewhere(int v) { return scaled(v); }
