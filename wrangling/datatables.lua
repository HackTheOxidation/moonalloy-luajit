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

const struct DataTable *datatable_read_from_csv(char *c_str);

]]

-- Load the shared library from '.so'-file
local rust_lib = ffi.load("./moonalloy/target/debug/libmoonalloy.so")

local dt

local dt_mt = {
  __index = dt
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

  local type = "char[" .. (#path + 1) .."]"

  self.data = rust_lib.datatable_read_from_csv(ffi.new(type, path))

  return self
end

return datatables
