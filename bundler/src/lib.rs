use core::fmt;

use wasm_encoder::{
    ConstExpr, DataCountSection, DataSection, ImportSection, LinkingSection, MemoryType, Module,
    SymbolTable,
};

pub struct BlobBundler {
    imports: ImportSection,
    data_count: DataCountSection,
    data: DataSection,
    mem_offset: u32,
    symbol_table: SymbolTable,
}

impl BlobBundler {
    pub fn new() -> Self {
        let mut imports = ImportSection::new();
        imports.import(
            "env",
            "memory",
            MemoryType { minimum: 1, maximum: None, memory64: false, shared: false },
        );

        Self {
            imports,
            data_count: DataCountSection { count: 0 },
            data: DataSection::new(),
            mem_offset: 0,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn blob(&mut self, name: &str, blob: &[u8]) -> Result<&mut Self, BlobTooBig> {
        let data_name = ["BLOB.DATA.", name].concat();
        let len_name = ["BLOB.LEN.", name].concat();
        self.raw_data(&data_name, blob, 1)?;
        self.raw_data(&len_name, &(blob.len() as u32).to_le_bytes(), 4)?;
        Ok(self)
    }

    /// Add a chunk of data to the module.
    ///
    /// Note that this only exports a single piece of data under the name `name`,
    /// so if it's possible that the data would have a variable size you'd
    /// probably rather use [`BlobBundler::blob()`].
    pub fn raw_data(
        &mut self,
        name: &str,
        data: &[u8],
        align: u32,
    ) -> Result<&mut Self, BlobTooBig> {
        let size: u32 = data.len().try_into().map_err(|_| BlobTooBig)?;
        let index = self.data.len();
        // add padding for the alignment of the data
        // based off Layout::padding_needed_for
        self.mem_offset =
            self.mem_offset.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
        self.data.active(0, &ConstExpr::i32_const(self.mem_offset as _), data.iter().copied());
        self.mem_offset += size;
        let data_def = wasm_encoder::DataSymbolDefinition { index, offset: 0, size };
        self.symbol_table.data(0, name, Some(data_def));
        self.data_count.count += 1;
        Ok(self)
    }

    pub fn build(&self) -> Vec<u8> {
        let mut module = Module::new();
        module
            .section(&self.imports)
            .section(&self.data_count)
            .section(&self.data)
            .section(LinkingSection::new().symbol_table(&self.symbol_table));
        module.finish()
    }
}

impl Default for BlobBundler {
    fn default() -> Self {
        Self::new()
    }
}

/// An error denoting that the data you tried to encode had a length longer than u32::MAX
#[derive(Debug)]
#[non_exhaustive]
pub struct BlobTooBig;

impl fmt::Display for BlobTooBig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("blob does not fit in a 32-bit address space")
    }
}
