extern crate crypto;
extern crate tempify;

use std::fs::File;
use std::io::prelude::*;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use self::crypto::sha2::Sha256;
use self::crypto::md5::Md5;

#[allow(unused_imports)]
use tempify::Temp;

/// calculate hash sum at once
#[derive(Debug, Clone)]
pub struct ContentIdentifier {
    pub sha256: String,
    pub sha1: String,
    pub md5: String,
}

impl ContentIdentifier {
    /// calculate hash sum of file
    pub fn of_file(filepath: &str) -> Result<ContentIdentifier, String> {
        let mut file: File;
        match File::open(filepath) {
            Ok(f) => file = f,
            Err(e) => return Err(e.to_string()),
        }

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
    pub fn of_str(string: &str) -> ContentIdentifier {
        let mut sha256er = Sha256::new();
        let mut sha1er = Sha1::new();
        let mut md5er = Md5::new();

        sha256er.input_str(string);
        sha1er.input_str(string);
        md5er.input_str(string);

        ContentIdentifier {
            sha256: sha256er.result_str(),
            sha1: sha1er.result_str(),
            md5: md5er.result_str(),
        }
    }
}

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

    test_write(tmp.path.as_str());
    let cid: ContentIdentifier;

    match ContentIdentifier::of_file(tmp.path.as_str()) {
        Ok(h) => cid = h,
        Err(e) => panic!(e.to_string()),
    }

    assert_eq!(
        cid.sha256,
        "185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969"
    );
    assert_eq!(cid.sha1, "f7ff9e8b7bb2e09b70935a5d785e0cc5d9d0abf0");
    assert_eq!(cid.md5, "8b1a9953c4611296a827abf8c47804d7");
}

#[allow(unused)]
fn test_write(path: &str) {
    let mut file = File::create(path).unwrap();
    let _ = file.write_all(b"Hello");
}
