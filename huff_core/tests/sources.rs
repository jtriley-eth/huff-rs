use std::path::PathBuf;

use huff_core::Compiler;
use huff_utils::files::FileSource;

#[test]
fn test_fetch_sources() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![
        PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string()),
        PathBuf::from("../huff-examples/erc20/contracts/utils/Address.huff".to_string()),
        PathBuf::from("../huff-examples/erc20/contracts/utils/HashMap.huff".to_string()),
    ]);
    assert_eq!(file_sources.len(), 3);
    assert_eq!(file_sources[0].path, "../huff-examples/erc20/contracts/ERC20.huff".to_string());
    assert_eq!(
        file_sources[1].path,
        "../huff-examples/erc20/contracts/utils/Address.huff".to_string()
    );
    assert_eq!(
        file_sources[2].path,
        "../huff-examples/erc20/contracts/utils/HashMap.huff".to_string()
    );
}

#[test]
fn test_fetch_invalid_sources() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![
        PathBuf::from("../huff-examples/erc20/contracts/non_existant.huff".to_string()),
        PathBuf::from("../huff-examples/erc20/contracts/non_huff.txt".to_string()),
        PathBuf::from("../huff-examples/erc20/contracts/random/Address.huff".to_string()),
        PathBuf::from("../huff-examples/erc20/contracts/random/".to_string()),
        PathBuf::from("../huff-examples/erc20/contracts/utils/".to_string()),
    ]);
    assert_eq!(file_sources.len(), 0);
}
