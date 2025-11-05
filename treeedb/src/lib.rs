use tree_sitter::Tree;

pub mod consumer;
pub mod narrow;
pub mod wide;

#[cfg(feature = "cli")]
pub mod cli;

pub fn facts<E>(
    fc: &mut impl consumer::FactConsumer<Err = E>,
    file_id: &str,
    source: &[u8],
    tree: Tree,
) -> Result<(), E> {
    let mut nodes = vec![tree.root_node()];
    let mut cursor = tree.walk();
    while let Some(node) = nodes.pop() {
        fc.node(file_id, &node, source)?;
        for (i, child) in node.children(&mut cursor).enumerate() {
            if let Some(name) = node.field_name_for_child(i as u32) {
                fc.field(file_id, &node, name, &child)?;
            }
        }
        for (i, child) in node.named_children(&mut cursor).enumerate() {
            fc.child(file_id, &node, i as u32, &child)?;
            nodes.push(child);
        }
    }
    Ok(())
}
