use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_dept};
use serde::Serialize;
use utils::tree::{get_tree_start_parent_id, vec_to_tree_into, Tree, TreeInfo};

pub async fn create(pool: &ConnectPool, params: system_dept::FormParamsForCreate) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::create(&mut conn, &params)
        .await?
        .into())
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_dept::FormParamsForCreate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::update(&mut conn, id, params)
        .await?
        .into())
}

pub async fn delete(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::soft_delete_transaction(&mut conn, id)
        .await?
        .into())
}

pub async fn get_dept_children_ids(pool: &ConnectPool, parent_dept_id: i32) -> Result<Vec<i32>> {
    let infos = get_dept_tree(pool, &Filter::default()).await?;
    let mut parent_dept_ids = vec![parent_dept_id];
    Ok(get_children_ids(infos, &mut parent_dept_ids).clone())
}

fn get_children_ids(tree: Vec<Dept>, parent_dept_ids: &mut Vec<i32>) -> &mut Vec<i32> {
    for dept in tree {
        if parent_dept_ids.contains(&dept.info.info.parent_id) {
            parent_dept_ids.push(dept.info.info.id);
        }
        if !dept.children.is_empty() {
            get_children_ids(dept.children, parent_dept_ids);
        }
    }
    parent_dept_ids
}

pub async fn get_user_dept_trees(pool: &ConnectPool, filter: Filter) -> Result<Vec<Dept>> {
    let infos = get_depts_by_user_id(pool, filter).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    println!("{:#?}", infos.len());
    Ok(vec_to_tree_into::<Dept, Info>(&parent_id, &infos))
}

pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::find(
        &mut conn,
        system_dept::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?
    .into())
}

pub async fn info_by_name(pool: &ConnectPool, name: &str) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dept::Entity::find(
        &mut conn,
        system_dept::Filter {
            name: Some(name.to_owned()),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?
    .into())
}

async fn get_dept_tree(pool: &ConnectPool, filter: &Filter) -> Result<Vec<Dept>> {
    let infos = get_depts(pool, filter.clone()).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);

    Ok(vec_to_tree_into::<Dept, Info>(&parent_id, &infos))
}

async fn get_depts(pool: &ConnectPool, filter: Filter) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;

    Ok(system_dept::Entity::query(&mut conn, filter)
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

async fn get_depts_by_user_id(pool: &ConnectPool, filter: Filter) -> Result<Vec<Info>> {
    let depts = get_depts(pool, filter.clone()).await?;
    Ok(get_children_dept(depts, filter.id))
}

fn get_children_dept(depts: Vec<Info>, dept_id: Option<i32>) -> Vec<Info> {
    let mut new_depts = vec![];
    for dept in depts.clone() {
        match dept_id {
            Some(id) => {
                if dept.get_parent_id().eq(&id) {
                    new_depts.push(dept.clone());
                }
            }
            None => {
                new_depts.push(dept.clone());
            }
        };
        let is_exist_children = depts
            .clone()
            .into_iter()
            .find(|x| x.get_parent_id().eq(&dept.get_id()))
            .is_some();
        if is_exist_children {
            let filter_self_depts = depts
                .clone()
                .into_iter()
                .filter(|x| x.get_id().ne(&dept.get_id()))
                .collect::<Vec<Info>>();
            let children = get_children_dept(filter_self_depts, Some(dept.get_id()));

            new_depts.extend(children);
        }
    }
    let mut new_new_depts = vec![];
    let mut new_dept_ids = vec![];
    for item in new_depts {
        if !new_dept_ids.contains(&item.get_id()) {
            new_dept_ids.push(item.get_id());
            new_new_depts.push(item);
        }
    }
    new_new_depts
}
#[derive(Debug, Clone, Serialize)]
pub struct Info {
    #[serde(flatten)]
    info: system_dept::Entity,
}
impl From<system_dept::Entity> for Info {
    fn from(value: system_dept::Entity) -> Self {
        Self { info: value }
    }
}
impl TreeInfo for Info {
    fn get_parent_id(&self) -> i32 {
        self.info.parent_id
    }

    fn get_id(&self) -> i32 {
        self.info.id
    }
}
#[derive(Debug, Serialize)]
pub struct Dept {
    #[serde(flatten)]
    info: Info,
    children: Vec<Dept>,
}

impl From<Info> for Dept {
    fn from(value: Info) -> Self {
        Self {
            info: value,
            children: vec![],
        }
    }
}
impl From<system_dept::Entity> for Dept {
    fn from(value: system_dept::Entity) -> Self {
        Self {
            info: value.into(),
            children: vec![],
        }
    }
}

impl Tree<Dept> for Dept {
    fn set_child(&mut self, data: Vec<Dept>) {
        self.children = data;
    }
}
#[derive(Debug, Default, Clone)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    pub id: Option<i32>,
    pub user_id: Option<i32>,
}
impl From<Filter> for system_dept::Filter {
    fn from(value: Filter) -> Self {
        Self {
            keyword: value.keyword,
            status: value.status,
            ..Default::default()
        }
    }
}
pub type FormParamsForCreate = system_dept::FormParamsForCreate;
