// tidal-rules: STG-15 STG-13
// tidal-expect: run
int main() {
  int items[3] = {1, 2, 3};
  long single = 4;
  const void* past_items = items + 3;
  const void* after_items = reinterpret_cast<unsigned char*>(items) + sizeof items;
  const void* past_single = &single + 1;
  const void* after_single = reinterpret_cast<unsigned char*>(&single) + sizeof single;
  const void* first = &single;
  const void* first_byte = reinterpret_cast<unsigned char*>(&single);
  return past_items == after_items && past_single == after_single && first == first_byte ? 0 : 1;
}
