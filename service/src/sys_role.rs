use crate::{
    prisma::system_role::{self, SetParam, UncheckedSetParam},
    Database, Result,
};

system_role::partial_unchecked!(RoleCreateParams {
    sort
    describe
    status
});

pub async fn create(
    client: &Database,
    name: &str,
    sign: &str,
    params: Vec<UncheckedSetParam>,
) -> Result<system_role::Data> {
    Ok(client
        .system_role()
        .create_unchecked(name.to_owned(), sign.to_owned(), params)
        .exec()
        .await?)
}

pub async fn upsert(
    client: &Database,
    name: &str,
    sign: &str,
    params: Vec<UncheckedSetParam>,
) -> Result<system_role::Data> {
    let data = params
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<SetParam>>();
    Ok(client
        .system_role()
        .upsert(
            system_role::sign::equals(sign.to_owned()),
            (name.to_owned(), sign.to_owned(), data.clone()),
            data,
        )
        .exec()
        .await?)
}
