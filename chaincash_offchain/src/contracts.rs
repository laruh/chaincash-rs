use ergo_lib::ergoscript_compiler::compiler::compile;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use once_cell::sync::Lazy;

static RESERVE_ERGO_TREE: Lazy<ErgoTree> = Lazy::new(|| {
    let s = include_str!("../../contracts/chaincash/contracts/initial/reserve.es");
    compile(s, Default::default()).unwrap()
});

static NOTE_ERGO_TREE: Lazy<ErgoTree> = Lazy::new(|| {
    let s = include_str!("../../contracts/chaincash/contracts/initial/note.es");
    compile(s, Default::default()).unwrap()
});
