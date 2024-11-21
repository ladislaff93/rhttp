use std::ops::{Deref, DerefMut};
use http::{common::{CRLF, FINAL_CRLF}, response::Response};
use std::io::Write;

pub struct Outcoming<'o>(Response<'o>);

impl <'o> Outcoming<'o> {
    pub fn new(resp: Response<'o>) -> Outcoming<'o> {
        Outcoming(resp)
    }
}

pub trait Serialize {
    fn serialize(self) -> Vec<u8>;
}

impl <'o> Deref for Outcoming<'o> {
    type Target=Response<'o>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'o> DerefMut for Outcoming<'o> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <'o> Serialize for Outcoming<'o> {
    fn serialize(self) -> Vec<u8> {
        let mut return_buff = vec![];

        write!(&mut return_buff, "{} {} {}{CRLF}", self.status_line.version, self.status_line.status_code, self.status_line.reason_phrase)
            .unwrap();

        for (idx, header) in self.headers.iter().enumerate() {
            let header_type = header.0;
            let header_value = header.1.to_str().unwrap();
            if idx == self.headers.len()-1 {
                write!(&mut return_buff, "{}: {}{}", header_type, header_value, FINAL_CRLF)
                    .unwrap();
            } else {
                write!(&mut return_buff, "{}: {}{}", header_type, header_value, CRLF)
                    .unwrap();
            }
        }

        write!(&mut return_buff, "{}", self.body)
            .unwrap();

        return_buff 
    }
}
