use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_dept};
use serde::Serialize;
use utils::tree::{get_tree_start_parent_id, vec_to_tree_into, Tree, TreeInfo};

pub async fn create(
    pool: &ConnectPool,
    params: &system_dept::FormParamsForCreate,
) -> Result<system_dept::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::create(&mut conn, params).await?)
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_dept::FormParamsForCreate,
) -> Result<system_dept::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::update(&mut conn, id, params).await?)
}

pub async fn delete(pool: &ConnectPool, id: i32) -> Result<system_dept::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::soft_delete_transaction(&mut conn, id).await?)
}

pub async fn get_dept_children_ids(pool: &ConnectPool, parent_dept_id: i32) -> Result<Vec<i32>> {
    let infos = get_dept_tree(
        pool,
        &system_dept::Filter {
            keyword: None,
            status: None,
            ..Default::default()
        },
    )
    .await?;
    let mut parent_dept_ids = vec![parent_dept_id];
    Ok(get_children_ids(infos, &mut parent_dept_ids).clone())
}

fn get_children_ids(tree: Vec<Dept>, parent_dept_ids: &mut Vec<i32>) -> &mut Vec<i32> {
    for dept in tree {
        if parent_dept_ids.contains(dept.info.parent_id()) {
            parent_dept_ids.push(dept.info.id());
        }
        if !dept.children.is_empty() {
            get_children_ids(dept.children, parent_dept_ids);
        }
    }
    parent_dept_ids
}

pub async fn get_user_dept_trees(
    pool: &ConnectPool,
    params: &system_dept::Filter,
) -> Result<Vec<Dept>> {
    let infos = get_depts_by_user_id(pool, params).await?;
    let parent_id = get_tree_start_parent_id::<system_dept::Entity>(&infos);
    Ok(vec_to_tree_into::<Dept, system_dept::Entity>(
        &parent_id, &infos,
    ))
}

pub async fn info(pool: &ConnectPool, id: i32) -> Result<system_dept::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::find(
        &mut conn,
        &system_dept::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?)
}

pub async fn info_by_name(pool: &ConnectPool, name: &str) -> Result<system_dept::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::find(
        &mut conn,
        &system_dept::Filter {
            name: Some(name.to_owned()),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?)
}

async fn get_dept_tree(pool: &ConnectPool, params: &system_dept::Filter) -> Result<Vec<Dept>> {
    let infos = get_depts(pool, params).await?;
    let parent_id = get_tree_start_parent_id::<system_dept::Entity>(&infos);
    Ok(vec_to_tree_into::<Dept, system_dept::Entity>(
        &parent_id, &infos,
    ))
}

async fn get_depts(
    pool: &ConnectPool,
    params: &system_dept::Filter,
) -> Result<Vec<system_dept::Entity>> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::query(&mut conn, params).await?)
}

async fn get_depts_by_user_id(
    pool: &ConnectPool,
    params: &system_dept::Filter,
) -> Result<Vec<system_dept::Entity>> {
    Ok(get_children_dept(get_depts(pool, params).await?, params.id))
}

fn get_children_dept(
    depts: Vec<system_dept::Entity>,
    dept_id: Option<i32>,
) -> Vec<system_dept::Entity> {
    let mut new_depts = vec![];
    for dept in depts.clone() {
        match dept_id {
            Some(id) => {
                if dept.parent_id().eq(&id) {
                    new_depts.push(dept.clone());
                }
            }
            None => {
                new_depts.push(dept.clone());
            }
        };
        let children = get_children_dept(depts.clone(), Some(dept.get_id()));
        new_depts.extend(children);
    }
    new_depts
}

#[derive(Debug, Serialize)]
pub struct Dept {
    #[serde(flatten)]
    info: system_dept::Entity,
    children: Vec<Dept>,
}

impl From<system_dept::Entity> for Dept {
    fn from(value: system_dept::Entity) -> Self {
        Self {
            info: value,
            children: vec![],
        }
    }
}

impl Tree<Dept> for Dept {
    fn set_child(&mut self, data: Vec<Dept>) {
        self.children = data;
    }
}
