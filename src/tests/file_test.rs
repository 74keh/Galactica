use crate::file::block_id::BlockId;
use crate::file::page::Page;
use crate::server::galactica::Galactica;

#[test]
fn file_test() {
    let db = Galactica::new("filetest", 400, 8);
    let file_manager = db.file_manager();

    let block_id = BlockId::new("testfile", 2);
    let mut page1 = Page::new(file_manager.block_size());
    let position1: usize = 88;
    page1.write_string(position1, "abcdefghijklm");
    let size = Page::max_length("abcdefghijklm".len());
    let position2 = position1 + size;
    page1.write_i32(position2, 345);
    file_manager.write(&block_id, &mut page1).unwrap();

    let mut page2 = Page::new(file_manager.block_size());
    file_manager.read(&block_id, &mut page2).unwrap();

    println!(
        "offset {} contains {}",
        position2,
        page2.read_i32(position2)
    );
    println!(
        "offset {} contains {}",
        position1,
        page2.read_string(position1).unwrap()
    );
}
