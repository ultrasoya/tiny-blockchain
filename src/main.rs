mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut tiny_blockchain = Blockchain::new();

    tiny_blockchain.add_block("Second block".to_string());
    tiny_blockchain.add_block("Third block".to_string());

    for block in tiny_blockchain.get_chain().iter() {
        println!("{:?}", block);
    }
}
