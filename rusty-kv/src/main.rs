mod datastruct;
use datastruct::linkedlist::Linkedlist;
use datastruct::btree::Tree;


fn main() {
    let mut tree: Tree<u8> = Tree::new();
    tree.set(65, 5);
    tree.set(75, 10);
    tree.set(15, 2);
    tree.set(55, 100);
    tree.set(65, 55);

    tree.pretty_print();
}
