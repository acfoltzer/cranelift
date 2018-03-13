//! Helper functions and structures for the translation.
use wasmparser;
use cretonne;
use std::u32;

/// Index of a function (imported or defined) inside the WebAssembly module.
pub type FunctionIndex = usize;
/// Index of a table (imported or defined) inside the WebAssembly module.
pub type TableIndex = usize;
/// Index of a global variable (imported or defined) inside the WebAssembly module.
pub type GlobalIndex = usize;
/// Index of a linear memory (imported or defined) inside the WebAssembly module.
pub type MemoryIndex = usize;
/// Index of a signature (imported or defined) inside the WebAssembly module.
pub type SignatureIndex = usize;

/// WebAssembly global.
#[derive(Debug, Clone, Copy)]
pub struct Global {
    /// The type of the value stored in the global.
    pub ty: cretonne::ir::Type,
    /// A flag indicating whether the value may change at runtime.
    pub mutability: bool,
    /// The source of the initial value.
    pub initializer: GlobalInit,
}

/// Globals are initialized via the four `const` operators or by referring to another import.
#[derive(Debug, Clone, Copy)]
pub enum GlobalInit {
    /// An `i32.const`.
    I32Const(i32),
    /// An `i64.const`.
    I64Const(i64),
    /// An `f32.const`.
    F32Const(u32),
    /// An `f64.const`.
    F64Const(u64),
    /// A `get_global` of another global.
    GlobalRef(GlobalIndex),
    ///< The global is imported from, and thus initialized by, a different module.
    Import(),
}

/// WebAssembly table.
#[derive(Debug, Clone, Copy)]
pub struct Table {
    /// The type of data stored in elements of the table.
    pub ty: TableElementType,
    /// The minimum number of elements in the table.
    pub size: usize,
    /// The maximum number of elements in the table.
    pub maximum: Option<usize>,
}

/// WebAssembly table element. Can be a function or a scalar type.
#[derive(Debug, Clone, Copy)]
pub enum TableElementType {
    Val(cretonne::ir::Type),
    Func(),
}

/// WebAssembly linear memory.
#[derive(Debug, Clone, Copy)]
pub struct Memory {
    /// The minimum number of pages in the memory.
    pub pages_count: usize,
    /// The maximum number of pages in the memory.
    pub maximum: Option<usize>,
    /// Whether the memory may be shared between multiple threads.
    pub shared: bool,
}

/// Helper function translating wasmparser types to Cretonne types when possible.
pub fn type_to_type(ty: &wasmparser::Type) -> Result<cretonne::ir::Type, ()> {
    match *ty {
        wasmparser::Type::I32 => Ok(cretonne::ir::types::I32),
        wasmparser::Type::I64 => Ok(cretonne::ir::types::I64),
        wasmparser::Type::F32 => Ok(cretonne::ir::types::F32),
        wasmparser::Type::F64 => Ok(cretonne::ir::types::F64),
        _ => Err(()),
    }
}

/// Turns a `wasmparser` `f32` into a `Cretonne` one.
pub fn f32_translation(x: wasmparser::Ieee32) -> cretonne::ir::immediates::Ieee32 {
    cretonne::ir::immediates::Ieee32::with_bits(x.bits())
}

/// Turns a `wasmparser` `f64` into a `Cretonne` one.
pub fn f64_translation(x: wasmparser::Ieee64) -> cretonne::ir::immediates::Ieee64 {
    cretonne::ir::immediates::Ieee64::with_bits(x.bits())
}

/// Translate a `wasmparser` type into its `Cretonne` equivalent, when possible
pub fn num_return_values(ty: wasmparser::Type) -> usize {
    match ty {
        wasmparser::Type::EmptyBlockType => 0,
        wasmparser::Type::I32 |
        wasmparser::Type::F32 |
        wasmparser::Type::I64 |
        wasmparser::Type::F64 => 1,
        _ => panic!("unsupported return value type"),
    }
}

/// Get the current source location from a reader.
pub fn cur_srcloc(reader: &wasmparser::BinaryReader) -> cretonne::ir::SourceLoc {
    // We record source locations as byte code offsets relative to the beginning of the function.
    // This will wrap around of a single function's byte code is larger than 4 GB, but a) the
    // WebAssembly format doesn't allow for that, and b) that would hit other Cretonne
    // implementation limits anyway.
    cretonne::ir::SourceLoc::new(reader.current_position() as u32)
}
