use crate::{
    now_time,
    prisma::{system_dept, system_role, SortOrder},
    sys_user, Database, Result, ServiceError, Tree,
};
use serde::Serialize;
use std::sync::Arc;

pub async fn create(
    client: &Database,
    name: String,
    params: DeptCreateParams,
) -> Result<system_dept::Data> {
    Ok(client
        .system_dept()
        .create_unchecked(name, params.to_params())
        .exec()
        .await?)
}

pub async fn update(
    client: &Database,
    id: i32,
    params: DeptUpdateParams,
) -> Result<system_dept::Data> {
    Ok(client
        .system_dept()
        .update_unchecked(system_dept::id::equals(id), params.to_params())
        .exec()
        .await?)
}

pub async fn delete(client: &Database, id: i32) -> Result<system_dept::Data> {
    let res = client
        ._transaction()
        .run::<ServiceError, _, _, _>(|client| async move {
            let client = Arc::new(client);
            let info = client
                .system_dept()
                .update(
                    system_dept::id::equals(id),
                    vec![system_dept::deleted_at::set(Some(now_time()))],
                )
                .exec()
                .await?;
            let user_ids = sys_user::get_users_by_dept_id(&client, id)
                .await?
                .into_iter()
                .map(|x| x.id)
                .collect::<Vec<i32>>();
            if !user_ids.is_empty() {
                sys_user::batch_set_dept(&client, None, user_ids).await?;
            }
            Ok(info)
        })
        .await?;
    Ok(res)
}

pub async fn get_user_dept_trees(client: &Database, user_id: i32) -> Result<Vec<Dept>> {
    Ok(tree::<Dept>(
        &0,
        get_depts_by_user_id(client, user_id).await?,
    ))
}

fn tree<T: Tree<T> + std::convert::From<Info>>(parent_id: &i32, menus: Vec<Info>) -> Vec<T> {
    menus
        .clone()
        .into_iter()
        .filter(|x| x.parent_id.eq(parent_id))
        .map(|x| {
            let mut data: T = x.clone().into();
            data.set_children(tree::<T>(&x.id, menus.clone()));
            data
        })
        .collect::<Vec<T>>()
}

async fn get_depts(client: &Database) -> Result<Vec<Info>> {
    Ok(client
        .system_dept()
        .find_many(vec![system_dept::deleted_at::equals(None)])
        .order_by(system_dept::id::order(SortOrder::Asc))
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

async fn get_depts_by_user_id(client: &Database, user_id: i32) -> Result<Vec<Info>> {
    Ok(
        match sys_user::get_current_user_info(client, user_id).await? {
            Some(user_permission) => match (user_permission.role, user_permission.dept) {
                (Some(role), Some(dept)) => {
                    let depts = get_role_dept(client, role).await?;
                    if !depts.is_empty() {
                        depts
                    } else {
                        get_children_dept(get_depts(client).await?, dept.id)
                    }
                }
                (None, Some(dept)) => get_children_dept(get_depts(client).await?, dept.id),
                (Some(role), None) => get_role_dept(client, role).await?,
                _ => vec![],
            },
            None => vec![],
        },
    )
}

async fn get_role_dept(client: &Database, role: system_role::Data) -> Result<Vec<Info>> {
    Ok(match role.sign.as_str() {
        super::ADMIN_ROLE_SIGN => get_depts(client).await?,
        _ => vec![],
    })
}

fn get_children_dept(depts: Vec<Info>, dept_id: i32) -> Vec<Info> {
    let mut new_depts = vec![];
    for dept in depts.clone() {
        if dept.parent_id.eq(&dept_id) {
            new_depts.push(dept.clone());
            let children = get_children_dept(depts.clone(), dept.id);
            new_depts.extend(children);
        }
    }
    new_depts
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

impl Tree<Dept> for Dept {
    fn set_children(&mut self, data: Vec<Dept>) {
        self.children = data;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    id: i32,
    parent_id: i32,
    name: String,
    person_name: String,
    person_phone: String,
    person_email: String,
    describe: String,
    status: bool,
}

impl From<system_dept::Data> for Info {
    fn from(value: system_dept::Data) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name,
            person_name: value.person_name,
            person_phone: value.person_phone,
            person_email: value.person_email,
            describe: value.describe,
            status: value.status,
        }
    }
}

system_dept::partial_unchecked!(DeptCreateParams {
    parent_id
    person_name
    person_phone
    person_email
    describe
    status
    sort
});

system_dept::partial_unchecked!(DeptUpdateParams {
    parent_id
    name
    person_name
    person_phone
    person_email
    describe
    status
    sort
});
