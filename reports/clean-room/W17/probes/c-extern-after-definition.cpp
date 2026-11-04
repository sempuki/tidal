// Probe: INST-26. An explicit instantiation declaration after the
// explicit instantiation definition. Expected: reject.
template <class T> T twice(T v) { return v + v; }
template int twice<int>(int);
extern template int twice<int>(int);
int main() { return twice(0); }
