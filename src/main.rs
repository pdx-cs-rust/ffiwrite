const O_WRONLY: isize = 0x1;
const O_CREAT: isize = 0x40;
const O_TRUNC: isize = 0x200;

extern "C" {
    fn open(pathname: *const u8, flags: isize, mode: isize) -> isize;
    fn write(fd: isize, buf: *const u8, count: usize) -> isize;
    fn close(fd: isize) -> isize;
}

fn main() {
    unsafe {
        let fd = open("/tmp/test\u{0}".as_bytes().as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o777);
        assert!(fd >= 0, "open failed");
        let buf = "hello world\n".as_bytes();
        let nwritten = write(fd, buf.as_ptr(), buf.len());
        assert!(nwritten >= 0, "write failed");
        assert!(nwritten == buf.len() as isize, "short write: {} < {}", nwritten, buf.len());
        let result = close(fd);
        assert!(result == 0, "close failed");
    }
}
