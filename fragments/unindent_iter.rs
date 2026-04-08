use std::fmt::Write;

pub trait UnindentIteratorExt {
    fn unindent(self) -> String;
}

impl<I, S> UnindentIteratorExt for I
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn unindent(self) -> String {
        let lines: Vec<_> = self.collect();

        let min_indent = lines
            .iter()
            .map(|s| s.as_ref())
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        let mut buf = String::new();

        for line in lines {
            let s = line.as_ref();
            if s.trim().is_empty() {
                writeln!(buf).unwrap();
            } else {
                writeln!(buf, "{}", s[min_indent..].trim_end()).unwrap();
            }
        }

        buf.trim().to_string()
    }
}
