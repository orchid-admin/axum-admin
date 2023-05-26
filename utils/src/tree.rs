pub trait Tree<T> {
    fn set_child(&mut self, data: Vec<T>);
}

pub trait TreeInfo {
    fn get_parent_id(&self) -> i32;
    fn get_id(&self) -> i32;
}

pub fn get_tree_start_parent_id<S>(infos: &[S]) -> i32
where
    S: TreeInfo,
{
    let mut parent_ids = infos
        .iter()
        .map(|x| x.get_parent_id())
        .collect::<Vec<i32>>();
    parent_ids.sort();
    let parent_id = parent_ids.first().copied().unwrap_or_default();
    parent_id
}
pub fn vec_to_tree_into<T, S>(parent_id: &i32, menus: &Vec<S>) -> Vec<T>
where
    T: Tree<T> + std::convert::From<S>,
    S: TreeInfo + Clone,
{
    menus
        .iter()
        .filter(|x| x.get_parent_id().eq(parent_id))
        .map(|node| {
            let node_id = node.get_id();
            let mut data: T = node.clone().into();
            data.set_child(vec_to_tree_into::<T, S>(&node_id, menus));
            data
        })
        .collect::<Vec<T>>()
}
