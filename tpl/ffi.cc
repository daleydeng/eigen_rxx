#include "wrapper.hh"

namespace eigen_rxx {
extern "C" {

{% for k, o in cffi_fns.items() -%}
{{ cffi_gen_fn(k, type_mapping=type_mapping, **o)["C"] }}
{% endfor %}

{% for k, o in cffi_mem_fns.items() -%}
{{ cffi_gen_mem_fn(k, type_mapping=type_mapping, **o)["C"] }}
{% endfor %}

} // extern "C"
} // namespace
