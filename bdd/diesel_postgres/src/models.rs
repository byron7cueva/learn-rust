use super::schemas::posts;

#[derive(Queryable)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool
}

#[derive(Insertable)]
#[table_name="pots"]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str
}