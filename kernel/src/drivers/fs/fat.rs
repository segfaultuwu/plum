use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::str::from_utf8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatType {
    Fat12,
    Fat16,
    Fat32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatError {
    InvalidBootSector,
    UnsupportedFatType,
    InvalidCluster,
    OutOfBounds,
    Utf8,
}

impl fmt::Display for FatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub struct FatVolume<'a> {
    image: &'a [u8],
    bytes_per_sector: usize,
    sectors_per_cluster: usize,
    reserved_sectors: usize,
    fats: usize,
    sectors_per_fat: usize,
    root_entry_count: usize,
    root_cluster: u32,
    fat_type: FatType,
    fat_start: usize,
    root_dir_start: usize,
    data_start: usize,
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_directory: bool,
    pub first_cluster: u32,
    pub size: u32,
}

impl<'a> FatVolume<'a> {
    pub fn new(image: &'a [u8]) -> Result<Self, FatError> {
        if image.len() < 64 {
            return Err(FatError::InvalidBootSector);
        }

        if image.get(510) != Some(&0x55) || image.get(511) != Some(&0xAA) {
            return Err(FatError::InvalidBootSector);
        }

        let bytes_per_sector = read_u16(image, 11)? as usize;
        let sectors_per_cluster =
            image.get(13).copied().ok_or(FatError::InvalidBootSector)? as usize;
        let reserved_sectors = read_u16(image, 14)? as usize;
        let fats = image.get(16).copied().ok_or(FatError::InvalidBootSector)? as usize;
        let root_entry_count = read_u16(image, 17)? as usize;
        let total_sectors_16 = read_u16(image, 19)? as usize;
        let sectors_per_fat_16 = read_u16(image, 22)? as usize;
        let total_sectors_32 = read_u32(image, 32)? as usize;
        let sectors_per_fat_32 = read_u32(image, 36)? as usize;

        let total_sectors = if total_sectors_16 != 0 {
            total_sectors_16
        } else {
            total_sectors_32
        };

        if bytes_per_sector == 0 || sectors_per_cluster == 0 || fats == 0 || total_sectors == 0 {
            return Err(FatError::InvalidBootSector);
        }

        let sectors_per_fat = if sectors_per_fat_16 != 0 {
            sectors_per_fat_16
        } else {
            sectors_per_fat_32
        };

        let root_dir_sectors =
            ((root_entry_count * 32) + (bytes_per_sector - 1)) / bytes_per_sector;
        let fat_start = reserved_sectors * bytes_per_sector;
        let root_dir_start = fat_start + fats * sectors_per_fat * bytes_per_sector;
        let data_start = root_dir_start + root_dir_sectors * bytes_per_sector;

        let root_cluster = if root_entry_count == 0 {
            read_u32(image, 44)?
        } else {
            0
        };

        let fat_type = if root_entry_count == 0 {
            FatType::Fat32
        } else if total_sectors < 4085 {
            FatType::Fat12
        } else if total_sectors < 65525 {
            FatType::Fat16
        } else {
            FatType::Fat32
        };

        Ok(Self {
            image,
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            fats,
            sectors_per_fat,
            root_entry_count,
            root_cluster,
            fat_type,
            fat_start,
            root_dir_start,
            data_start,
        })
    }

    pub fn fat_type(&self) -> FatType {
        self.fat_type
    }

    pub fn list_root_directory(&self) -> Result<Vec<DirectoryEntry>, FatError> {
        match self.fat_type {
            FatType::Fat12 | FatType::Fat16 => self.list_fixed_root_directory(),
            FatType::Fat32 => self.list_cluster_chain(self.root_cluster),
        }
    }

    // Public wrapper to list entries in a cluster chain (directory)
    pub fn list_directory_cluster(
        &self,
        start_cluster: u32,
    ) -> Result<Vec<DirectoryEntry>, FatError> {
        self.list_cluster_chain(start_cluster)
    }

    // Public wrapper to read a cluster chain's bytes (file contents)
    pub fn read_cluster_chain_bytes(
        &self,
        start_cluster: u32,
        size: u32,
    ) -> Result<Vec<u8>, FatError> {
        self.read_cluster_chain(start_cluster, size)
    }

    pub fn read_file(&self, name: &str) -> Result<Option<Vec<u8>>, FatError> {
        for entry in self.list_root_directory()? {
            if entry.is_directory {
                continue;
            }

            if entry.name.eq_ignore_ascii_case(name) {
                return self
                    .read_cluster_chain(entry.first_cluster, entry.size)
                    .map(Some);
            }
        }

        Ok(None)
    }

    fn list_fixed_root_directory(&self) -> Result<Vec<DirectoryEntry>, FatError> {
        let mut entries = Vec::new();
        let root_dir_bytes = self.root_entry_count * 32;
        let end = self.root_dir_start + root_dir_bytes;

        if end > self.image.len() {
            return Err(FatError::OutOfBounds);
        }

        let mut offset = self.root_dir_start;
        while offset + 32 <= end {
            if let Some(entry) = parse_directory_entry(&self.image[offset..offset + 32])? {
                entries.push(entry);
            }
            offset += 32;
        }

        Ok(entries)
    }

    fn list_cluster_chain(&self, start_cluster: u32) -> Result<Vec<DirectoryEntry>, FatError> {
        let mut entries = Vec::new();
        let mut cluster = start_cluster;

        while is_valid_cluster(cluster) {
            let cluster_bytes = self.cluster_bytes(cluster)?;
            let mut offset = 0;

            while offset + 32 <= cluster_bytes.len() {
                if let Some(entry) = parse_directory_entry(&cluster_bytes[offset..offset + 32])? {
                    entries.push(entry);
                }
                offset += 32;
            }

            let next = self.next_cluster(cluster)?;
            if next >= 0x0FFF_FFF8 || next == 0 {
                break;
            }
            cluster = next;
        }

        Ok(entries)
    }

    fn read_cluster_chain(&self, start_cluster: u32, size: u32) -> Result<Vec<u8>, FatError> {
        let mut remaining = size as usize;
        let mut cluster = start_cluster;
        let mut output = Vec::with_capacity(remaining);

        while remaining > 0 && is_valid_cluster(cluster) {
            let cluster_bytes = self.cluster_bytes(cluster)?;
            let take = remaining.min(cluster_bytes.len());
            output.extend_from_slice(&cluster_bytes[..take]);
            remaining -= take;

            let next = self.next_cluster(cluster)?;
            if next >= 0x0FFF_FFF8 || next == 0 {
                break;
            }
            cluster = next;
        }

        output.truncate(size as usize);
        Ok(output)
    }

    fn cluster_bytes(&self, cluster: u32) -> Result<&'a [u8], FatError> {
        let cluster_index = cluster.checked_sub(2).ok_or(FatError::InvalidCluster)? as usize;
        let cluster_size = self.bytes_per_sector * self.sectors_per_cluster;
        let offset = self.data_start + cluster_index * cluster_size;
        let end = offset + cluster_size;

        self.image.get(offset..end).ok_or(FatError::OutOfBounds)
    }

    fn next_cluster(&self, cluster: u32) -> Result<u32, FatError> {
        let fat_offset = match self.fat_type {
            FatType::Fat12 => (cluster as usize * 3) / 2,
            FatType::Fat16 => cluster as usize * 2,
            FatType::Fat32 => cluster as usize * 4,
        };

        let offset = self.fat_start + fat_offset;
        match self.fat_type {
            FatType::Fat12 => {
                let bytes = self
                    .image
                    .get(offset..offset + 2)
                    .ok_or(FatError::OutOfBounds)?;
                let value = if cluster & 1 == 0 {
                    u16::from_le_bytes([bytes[0], bytes[1]]) & 0x0FFF
                } else {
                    (u16::from_le_bytes([bytes[0], bytes[1]]) >> 4) & 0x0FFF
                };
                Ok(value as u32)
            }
            FatType::Fat16 => {
                let value = read_u16(self.image, offset)?;
                Ok(value as u32)
            }
            FatType::Fat32 => {
                let value = read_u32(self.image, offset)? & 0x0FFF_FFFF;
                Ok(value)
            }
        }
    }
}

fn parse_directory_entry(raw: &[u8]) -> Result<Option<DirectoryEntry>, FatError> {
    if raw.first() == Some(&0x00) {
        return Ok(None);
    }

    if raw.first() == Some(&0xE5) {
        return Ok(None);
    }

    if raw.get(11) == Some(&0x0F) {
        return Ok(None);
    }

    let name = format_fat_name(raw)?;
    let is_directory = raw.get(11).copied().unwrap_or_default() & 0x10 != 0;
    let first_cluster_low = read_u16(raw, 26)? as u32;
    let first_cluster_high = read_u16(raw, 20)? as u32;
    let first_cluster = (first_cluster_high << 16) | first_cluster_low;
    let size = read_u32(raw, 28)?;

    Ok(Some(DirectoryEntry {
        name,
        is_directory,
        first_cluster,
        size,
    }))
}

fn format_fat_name(raw: &[u8]) -> Result<String, FatError> {
    let name =
        from_utf8(raw.get(0..8).ok_or(FatError::OutOfBounds)?).map_err(|_| FatError::Utf8)?;
    let ext =
        from_utf8(raw.get(8..11).ok_or(FatError::OutOfBounds)?).map_err(|_| FatError::Utf8)?;

    let base = name.trim_end();
    let ext = ext.trim_end();

    let mut name = String::from(base);
    if !ext.is_empty() {
        name.push('.');
        name.push_str(ext);
    }

    Ok(name)
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, FatError> {
    let bytes = data.get(offset..offset + 2).ok_or(FatError::OutOfBounds)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, FatError> {
    let bytes = data.get(offset..offset + 4).ok_or(FatError::OutOfBounds)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn is_valid_cluster(cluster: u32) -> bool {
    cluster >= 2 && cluster < 0x0FFF_FFF8
}
