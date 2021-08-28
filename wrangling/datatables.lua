local datatables = {{}}

-- Load the FFI module
local ffi = require("ffi")

-- Define the structs and functions to search for in the shared library
ffi.cdef[[

typedef enum DataCell_Tag {
  Int,
  Float,
  Bool,
  Str,
  Empty,
} DataCell_Tag;

typedef struct DataCell {
  DataCell_Tag tag;
  union {
    struct {
      int32_t int_;
    };
    struct {
      double float_;
    };
    struct {
      bool bool_;
    };
    struct {
      char *str;
    };
  };
} DataCell;

typedef struct DataRow {
  uintptr_t length;
  const struct DataCell *entries;
} DataRow;

typedef struct DataTable {
  uintptr_t rows;
  uintptr_t cols;
  char **labels;
  const struct DataRow *data;
} DataTable;

struct DataTable *datatable_read_from_csv(char *c_str);
char* datatable_to_string(DataTable* dt);
char* datatable_get_labels(DataTable* dt);
]]

-- Load the shared library from '.so'-file
local rust_lib = ffi.load("./moonalloy/target/debug/libmoonalloy.so")

local dt

local dt_mt = {
  __index = dt,
  __tostring = function(d) return ffi.string(rust_lib.datatable_to_string(d)) end,
}

dt = ffi.metatype("DataTable", dt_mt)

-- DataTable Wrapper Class
DataTable = { rows = 0, cols = 0, labels = {}, data = nil }
DataTable.__index = DataTable

setmetatable(DataTable, {
    __call = function (cls, ...)
        return cls.from_csv(...)
    end,
})

function DataTable.from_csv(path)
  local self = setmetatable({}, DataTable)

  if type(path) ~= "string" then
    print("ERROR - Invalid path: must be a string.")
    return nil
  end

  local t = "char[" .. (#path + 1) .."]"
  self.data = rust_lib.datatable_read_from_csv(ffi.new(t, path))

  return self
end

function DataTable:tostring()
  return tostring(self)
end

function DataTable:get_labels()
  return ffi.string(rust_lib.datatable_get_labels(self.data))
end

DataTable.__tostring = function(d)
  return tostring(d.data)
end

return datatables
