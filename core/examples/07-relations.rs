#[derive(Clone, Debug, elephantry::Entity)]
pub struct Post {
    title: String,
    content: String,
    comments: Vec<String>,
}

mod post {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub id: u32,
        pub title: String,
        pub content: String,
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
    }

    pub struct Model<'a> {
        connection: &'a elephantry::Connection,
    }

    impl<'a> elephantry::Model<'a> for Model<'a> {
        type Entity = Entity;
        type Structure = Structure;

        fn new(connection: &'a elephantry::Connection) -> Self {
            Self { connection }
        }

        fn create_projection() -> elephantry::Projection {
            Self::default_projection()
                .unset_field("post_id")
                .add_field("id", "%:post_id:%")
        }
    }

    impl<'a> Model<'a> {
        pub fn find_with_comments(&self, id: i32) -> elephantry::Result<super::Post> {
            use elephantry::{Model, Structure};

            let query = r#"
select {projection}
    from {post}
    join {comment} using(post_id)
    where post_id = $1
    group by post_id, comment.created_at
    order by comment.created_at;
"#;

            let projection = Self::create_projection()
                .unset_field("post_id")
                .add_field("comments", "array_agg(comment.content)");

            let sql = query
                .replace("{projection}", &projection.to_string())
                .replace("{post}", <Self as elephantry::Model>::Structure::relation())
                .replace("{comment}", super::comment::Structure::relation());

            Ok(self
                .connection
                .query::<super::Post>(&sql, &[&id])?
                .nth(0)
                .unwrap())
        }
    }

    pub struct Structure;

    impl elephantry::Structure for Structure {
        fn relation() -> &'static str {
            "post"
        }

        fn primary_key() -> &'static [&'static str] {
            &["id"]
        }

        fn definition() -> &'static [&'static str] {
            &["id", "title", "content", "created_at"]
        }
    }
}

mod comment {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub id: u32,
        pub content: String,
        pub post_id: u32,
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
    }

    pub struct Model;

    impl<'a> elephantry::Model<'a> for Model {
        type Entity = Entity;
        type Structure = Structure;

        fn new(_: &'a elephantry::Connection) -> Self {
            Self {}
        }
    }

    pub struct Structure;

    impl elephantry::Structure for Structure {
        fn relation() -> &'static str {
            "comment"
        }

        fn primary_key() -> &'static [&'static str] {
            &["id"]
        }

        fn definition() -> &'static [&'static str] {
            &["id", "content", "post_id", "created_at"]
        }
    }
}

fn main() -> elephantry::Result<()> {
    pretty_env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;

    setup(&elephantry)?;
    let post_with_comment = elephantry.model::<post::Model>().find_with_comments(1)?;
    dbg!(post_with_comment);
    tear_down(&elephantry)?;

    Ok(())
}

fn setup(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    elephantry.execute(
        "
begin;

create temporary table post (
    post_id serial primary key,
    title text not null,
    content text not null,
    created_at timestamptz not null default now()
);

create temporary table comment (
    comment_id serial primary key,
    content text not null,
    post_id integer references post(post_id),
    created_at timestamptz not null default now()
);

insert into post (post_id, title, content) values(1, 'First post', 'lorem ipsum');
insert into comment (content, post_id) values('First comment', 1), ('Second comment', 1);

commit;",
    )?;

    Ok(())
}

fn tear_down(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    elephantry.execute("drop table comment;")?;
    elephantry.execute("drop table post;")?;

    Ok(())
}
