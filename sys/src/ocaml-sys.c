#include <caml/mlvalues.h>

void caml_sys_store_double_val(value x, double f) { Store_double_val(x, f); }
double caml_sys_double_val(value x) { return Double_val(x); }

double caml_sys_double_field(value x, mlsize_t i) { return Double_field(x, i); }

void caml_sys_store_double_field(value x, mlsize_t index, double d) {
  Store_double_field(x, index, d);
}
