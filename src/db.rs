use std::ops::Add;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::eval::Statement;

const NAME_SIZE: u32 = 32;
const EMAIL_SIZE: u32 = 255;
const ROW_SIZE: u32 = NAME_SIZE + EMAIL_SIZE + 4;
const PAGE_SIZE: u32 = 4096;
const ID_OFFSET: u32 = 0;
const ID_SIZE: u32 = 4;
const NAME_OFFSET: u32 = ID_OFFSET + 4;
const EMAIL_OFFSET: u32 = NAME_OFFSET + NAME_SIZE;
const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_PAGES: u32 = 100;
const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;


type Pages = [Option<Page>; TABLE_MAX_PAGES as usize];
#[derive(Debug)]
struct Page {
    data: [u8; PAGE_SIZE as usize],
    offset: u32,
}

#[derive(Debug)]
pub struct Row {
    pub(crate) id: u32,
    pub(crate) name: [u8; NAME_SIZE as usize],
    pub(crate) email: [u8; EMAIL_SIZE as usize],
}

pub struct Table {
    pages: Pages,
    num_rows: u32
}

impl Table {
    pub fn new() -> Table {
        Self {
            pages: [const { None }; TABLE_MAX_PAGES as usize],
            num_rows: 0,
        }
    }

    pub(crate) fn execute_insert(&mut self, statement: &Statement) -> Result<(), &'static str> {
        if self.num_rows >= TABLE_MAX_ROWS {
            return Err("Table full");
        }
        let row_to_insert = &statement.row_to_insert;
        let page = row_slot(self, self.num_rows);
        serialize_row(row_to_insert, page);

        self.num_rows = self.num_rows + 1;
        Ok(())
    }

    pub fn execute_select(&self) {
        println!("num_rows: {}", self.num_rows);
        for i in 0..self.num_rows {
            let current_page = (i / ROWS_PER_PAGE) as usize;
            let row_offset = i % ROWS_PER_PAGE;
            let byte_offset = (row_offset * ROW_SIZE) as usize;
            let page = self.pages[current_page].as_ref().unwrap();
            println!("querying start: {}, end: {}", byte_offset, byte_offset + ROW_SIZE as usize);
            let row = deserialize_row(&page.data[byte_offset..byte_offset + ROW_SIZE as usize]);
            println!("row: {:?}", row);
        }
    }
}

fn serialize_row(source: &Row, destination: &mut Page) {
    // to_le_bytes() converts the integer to little-endian bytes
    // TODO: check more about little-endian
    let id_starting_offset =  destination.offset as usize;
    let id_ending_offset = id_starting_offset + ID_SIZE as usize;

    let name_starting_offset = id_ending_offset;
    let name_ending_offset = name_starting_offset + NAME_SIZE as usize;

    destination.data[id_starting_offset..id_ending_offset].copy_from_slice(&source.id.to_le_bytes());
    destination.data[name_starting_offset..name_ending_offset].copy_from_slice(&source.name);
    println!("writing to start: {}, end: {}", id_starting_offset, name_ending_offset);
}

fn deserialize_row(source: &[u8]) -> Row {
    let mut id_bytes = [0u8; 4];
    id_bytes.copy_from_slice(&source[0.. 4]);
    let id = u32::from_le_bytes(id_bytes);

    let mut name = [0u8; NAME_SIZE as usize];
    let starting_name_offset = NAME_OFFSET as usize;
    let ending_name_offset = starting_name_offset + NAME_SIZE as usize;
    name.copy_from_slice(&source[starting_name_offset..ending_name_offset]);

    // let mut email = [0u8; EMAIL_SIZE];
    // email.copy_from_slice(&source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE]);
    //
    Row { id, name, email: [0u8; 255] }
}

fn row_slot(mut table: &mut Table, row_num: u32) -> &mut Page {
    let page_num = row_num / ROWS_PER_PAGE;
    let row_offset = row_num % ROWS_PER_PAGE;
    let byte_offset = row_offset * ROW_SIZE;
    println!("page_num: {}, row_offset: {}, byte_offset: {}", page_num, row_offset, byte_offset);
    // why cannot move?
    let page = table.pages[page_num as usize]
        .get_or_insert_with(|| Page {
            data: [0; 4096],
            offset: byte_offset,
        });
    page.offset = byte_offset;
    println!("new offset: {}", page.offset);
    page
}