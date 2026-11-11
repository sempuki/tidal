// tidal-rules: STG-29 ADD-14
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction typed_step_over_byte_array_distance.cpp. Q-0430. first is an int* that points to the first byte of pool (STG-29), and first + 1 is one past that byte as an array of one int (ADD-14). The provisional answer puts the position sizeof(int) bytes on, so its address, cast back to a byte pointer, is element sizeof(int) of pool.
alignas(int) unsigned char pool[4 * sizeof(int)];
int main() {
  int* first = static_cast<int*>(static_cast<void*>(pool));
  unsigned char* next = static_cast<unsigned char*>(static_cast<void*>(first + 1));
  return next - pool == static_cast<decltype(next - pool)>(sizeof(int)) ? 0 : 1;
}
