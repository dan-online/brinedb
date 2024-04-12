use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::brine)]
pub struct Brine {
  pub key: String,
  pub val: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::brine, treat_none_as_null = true, treat_none_as_default_value = false)]
pub struct NewBrine<'a> {
  pub key: &'a str,
  pub val: &'a str,
}
