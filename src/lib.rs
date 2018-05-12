extern crate crypto;

use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

use self::crypto::digest::Digest;
use self::crypto::md5::Md5;
use self::crypto::sha1::Sha1;
use self::crypto::sha2::Sha256;

/// calculate hash sum at once
#[derive(Debug, Clone)]
pub struct ContentIdentifier {
    pub sha256: String,
    pub sha1: String,
    pub md5: String,
}

impl ContentIdentifier {
    /// calculate hash sum of file
    pub fn of_file<S: AsRef<str>>(filepath: S) -> Result<ContentIdentifier> {
        let mut file = File::open(filepath.as_ref())?;

        let mut sha256er = Sha256::new();
        let mut sha1er = Sha1::new();
        let mut md5er = Md5::new();

        let mut buffer = Vec::new();

        let _ = file.read_to_end(&mut buffer);

        sha256er.input(buffer.as_mut_slice());
        sha1er.input(buffer.as_mut_slice());
        md5er.input(buffer.as_mut_slice());

        Ok(ContentIdentifier {
            sha256: sha256er.result_str(),
            sha1: sha1er.result_str(),
            md5: md5er.result_str(),
        })
    }

    /// calculate ContentIdentifiersum of string
    pub fn of_str<S: AsRef<str>>(content: S) -> ContentIdentifier {
        let mut sha256er = Sha256::new();
        let mut sha1er = Sha1::new();
        let mut md5er = Md5::new();

        sha256er.input_str(content.as_ref());
        sha1er.input_str(content.as_ref());
        md5er.input_str(content.as_ref());

        ContentIdentifier {
            sha256: sha256er.result_str(),
            sha1: sha1er.result_str(),
            md5: md5er.result_str(),
        }
    }
}

#[cfg(test)]
mod test {

    extern crate tempify;

    use super::*;
    use std::fs::File;

    use self::tempify::Temp;

    #[test]
    fn string_file_identifier_test() {
        let cid = ContentIdentifier::of_str("");
        assert_eq!(
            cid.sha256,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(cid.sha1, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(cid.md5, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn file_fid_test() {
        let tmp = Temp::new().unwrap();

        {
            let mut file = File::create(tmp.path.as_str()).unwrap();
            let _ = file.write_all(b"Hello");
        }

        let cid = ContentIdentifier::of_file(tmp.path.as_str()).unwrap();

        assert_eq!(
            cid.sha256,
            "185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969"
        );

        assert_eq!(cid.sha1, "f7ff9e8b7bb2e09b70935a5d785e0cc5d9d0abf0");
        assert_eq!(cid.md5, "8b1a9953c4611296a827abf8c47804d7");
    }

}
