use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub body: String,
}

impl std::fmt::Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Post {\n")?;
        f.write_fmt(format_args!("    id: {},\n", self.id))?;
        f.write_fmt(format_args!("    user_id: {},\n", self.user_id))?;
        f.write_fmt(format_args!("    title: {},\n", self.title))?;
        f.write_fmt(format_args!("    body: \n{}\n", self.body))?;
        f.write_str("}")
    }
}
