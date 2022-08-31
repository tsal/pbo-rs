#[test]
fn read() {
    use std::io::Cursor;
    let (header, _) = crate::header::PBOHeader::read(&mut Cursor::new(String::from(
        "images\\mission.jpg             ��*\\*W i",
    )))
        .unwrap();
    assert_eq!(header.filename, "images\\mission.jpg");
    assert_eq!(header.size, 1_546_304_959);
    assert_eq!(header.timestamp, 4_022_190_063);
}
