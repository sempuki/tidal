struct Table {
  int size;
  Table();
};
bool table_ready;
Table::Table() : size(28) { table_ready = true; }
Table table;
int table_slots() { return table_ready ? table.size : 0; }
