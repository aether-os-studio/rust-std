use super::*;

#[test]
fn read_vectored_at() {
    let msg = b"preadv is working!";
    let dir = crate::test_helpers::tmpdir();

    let filename = dir.join("preadv.txt");
    {
        let mut file = fs::File::create(&filename).unwrap();
        file.write_all(msg).unwrap();
    }
    {
        let file = fs::File::open(&filename).unwrap();
        let mut buf0 = [0; 4];
        let mut buf1 = [0; 3];

        let mut iovec = [
            io::IoSliceMut::new(&mut buf0),
            io::IoSliceMut::new(&mut buf1),
        ];

        let n = file.read_vectored_at(&mut iovec, 4).unwrap();

        assert!(n == 4 || n == 7);
        assert_eq!(&buf0, b"dv i");

        if n == 7 {
            assert_eq!(&buf1, b"s w");
        }
    }
}

#[test]
fn write_vectored_at() {
    let msg = b"pwritev is not working!";
    let dir = crate::test_helpers::tmpdir();

    let filename = dir.join("preadv.txt");
    {
        let mut file = fs::File::create(&filename).unwrap();
        file.write_all(msg).unwrap();
    }
    let expected = {
        let file = fs::File::options().write(true).open(&filename).unwrap();
        let buf0 = b"    ";
        let buf1 = b"great  ";

        let iovec = [io::IoSlice::new(buf0), io::IoSlice::new(buf1)];

        let n = file.write_vectored_at(&iovec, 11).unwrap();

        assert!(n == 4 || n == 11);

        if n == 4 {
            b"pwritev is     working!"
        } else {
            b"pwritev is     great  !"
        }
    };

    let content = fs::read(&filename).unwrap();
    assert_eq!(&content, expected);
}

#[test]
fn test_mkfifo() {
    let tmp_dir = crate::test_helpers::tmpdir();

    let fifo = tmp_dir.path().join("fifo");

    mkfifo(&fifo, Permissions::from_mode(0o774)).unwrap();

    let mut wx = fs::File::options()
        .read(true)
        .write(true)
        .open(&fifo)
        .unwrap();
    let mut rx = fs::File::open(fifo).unwrap();

    wx.write_all(b"hello, world!").unwrap();
    drop(wx);

    let mut s = String::new();
    rx.read_to_string(&mut s).unwrap();

    assert_eq!(s, "hello, world!");
}
