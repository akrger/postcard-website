use async_graphql::connection::query_with;
use async_graphql::connection::CursorType;
use async_graphql::connection::{query, Connection, EmptyFields, PageInfo};
use async_graphql::guard::Guard;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Data, EmptyMutation, Object, Result, Schema, Subscription};
use async_graphql::{OutputType, SimpleObject};
use async_graphql_warp::{graphql_subscription_with_data, Response};
use async_redis_session::RedisSessionStore;
use futures::{stream, Stream};
use sqlx::sqlite::SqlitePoolOptions;
use std::convert::Infallible;
use std::io;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use warp::{http::Response as HttpResponse, Filter};
use warp_sessions::{CookieOptions, MemoryStore, SameSiteCookieOption, Session, SessionWithStore};

struct MyToken(String);

struct QueryRoot;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    Admin,
    Guest,
}

struct RoleGuard {
    role: Role,
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}

#[derive(SimpleObject, Clone)]
struct Diff {
    diff: i32,
    cool: String,
}

use async_graphql::connection::Edge;
use sqlx::{Pool, Sqlite, SqliteConnection, SqlitePool};

#[derive(SimpleObject)]
struct MyConnection<T: Sync + std::marker::Send + OutputType> {
    edges: Vec<Edge<usize, T, EmptyFields>>,
    totalCount: usize,
    page_info: PageInfo,
}

#[Object]
impl QueryRoot {
    #[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn get_data<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        let cookie = ctx.data_opt::<Option<String>>().unwrap();
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
    async fn login<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        let mut x: RwLockWriteGuard<Session> =
            ctx.data::<Arc<RwLock<Session>>>().unwrap().write().unwrap();
        let bbb = x.insert("user", "1".to_string()).unwrap();
        Some("100")
    }

    async fn numbers(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<MyConnection<Post>> {
        let x = ctx.data::<SqlitePool>().unwrap();

        query_with(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let totalCount: (u32,) = sqlx::query_as("SELECT COUNT(*) from posts")
                    .fetch_one(x)
                    .await
                    .unwrap();
                let mut rows: Vec<Post> = vec![];
                let mut has_previous_page = false;
                let mut has_next_page = false;
                let mut start_cursor = 0;
                let mut end_cursor = 0;
                let mut should_reverse = false;
                match (after, before, first, last) {
                    (_, _, Some(first), Some(last)) => {
                        return Err("not allowed".into());
                    } // not allowed
                    (Some(after), Some(before), Some(first), _) => {
                        return Err("not allowed".into());
                    }
                    (Some(after), Some(before), _, Some(last)) => {
                        return Err("not allowed".into());
                    }
                    (Some(after), Some(before), None, None) => {
                        return Err("not allowed".into());
                    }
                    (None, Some(before), Some(first), None) => {
                        return Err("not allowed".into());
                    }
                    (Some(after), None, None, None) => {
                        return Err("not allowed".into());
                    }
                    (None, Some(before), None, None) => {
                        return Err("not allowed".into());
                    }
                    (Some(after), None, None, Some(last)) => {
                        return Err("not allowed".into());
                    }
                    (None, None, Some(first), None) => {
                        rows = sqlx::query_as::<_, Post>(
                            format!(
                                "SELECT * from {table} order by id ASC LIMIT ?;",
                                table = "posts"
                            )
                            .as_str(),
                        )
                        .bind(first as u32)
                        .fetch_all(x)
                        .await
                        .unwrap();
                        has_previous_page = false;
                        has_next_page = totalCount.0 as usize > first;
                        if let Some(i) = rows.first() {
                            start_cursor = (i.id) as usize;
                        }
                        if let Some(i) = rows.last() {
                            end_cursor = (i.id) as usize;
                        }
                    }
                    (None, None, None, Some(last)) => {
                        rows = sqlx::query_as::<_, Post>(
                            format!(
                                "SELECT * from {table} order by id DESC LIMIT ?;",
                                table = "posts"
                            )
                            .as_str(),
                        )
                        .bind(last as u32)
                        .fetch_all(x)
                        .await
                        .unwrap();
                        should_reverse = true;
                        has_previous_page = totalCount.0 as usize > last;
                        has_next_page = false;
                        if let Some(i) = rows.first() {
                            end_cursor = (i.id) as usize;
                        }
                        if let Some(i) = rows.last() {
                            start_cursor = (i.id) as usize;
                        }
                    }
                    (Some(after), None, Some(first), None) => {
                        let _has_previous_page: (bool,) =
                            sqlx::query_as("SELECT COUNT(*) from posts where id <= ?")
                                .bind(after as u32)
                                .fetch_one(x)
                                .await
                                .unwrap();
                        has_next_page = totalCount.0 as usize > first;
                        has_previous_page = _has_previous_page.0;
                        rows = sqlx::query_as::<_, Post>(
                            format!(
                                "SELECT * from {table} where id > ? order by id ASC LIMIT ?;",
                                table = "posts"
                            )
                            .as_str(),
                        )
                        .bind(after as u32)
                        .bind(first as u32)
                        .fetch_all(x)
                        .await
                        .unwrap();
                        if let Some(i) = rows.first() {
                            start_cursor = (i.id) as usize;
                        }
                        if let Some(i) = rows.last() {
                            end_cursor = (i.id) as usize;
                        }
                    }
                    (None, Some(before), None, Some(last)) => {
                        let _has_next_page: (bool,) =
                            sqlx::query_as("SELECT COUNT(*) from posts where id >= ?")
                                .bind(before as u32)
                                .fetch_one(x)
                                .await
                                .unwrap();
                        has_next_page = _has_next_page.0;
                        has_previous_page = (totalCount.0 as usize) > last;
                        rows = sqlx::query_as::<_, Post>(
                            format!(
                                "SELECT * from {table} where id < ? order by id DESC LIMIT ?;",
                                table = "posts"
                            )
                            .as_str(),
                        )
                        .bind(before as u32)
                        .bind(last as u32)
                        .fetch_all(x)
                        .await
                        .unwrap();
                        should_reverse = true;
                    }
                    _ => {
                        rows = sqlx::query_as::<_, Post>(
                            format!("SELECT * from {table} order by id ASC;", table = "posts")
                                .as_str(),
                        )
                        .fetch_all(x)
                        .await
                        .unwrap();
                        if let Some(i) = rows.first() {
                            start_cursor = (i.id) as usize;
                        }
                        if let Some(i) = rows.last() {
                            end_cursor = (i.id) as usize;
                        }
                    } // return everything
                };
                let mut edges = rows
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| Edge::new(item.id as usize, item))
                    .collect::<Vec<Edge<usize, Post, EmptyFields>>>();
                if should_reverse {
                    edges.reverse();
                }
                // before.unwrap_or((totalCount.0 - 1) as usize);
                // Won't work without this line O_O
                after.unwrap_or(0);
                let mut connection = MyConnection {
                    edges,
                    totalCount: totalCount.0 as usize,
                    page_info: PageInfo {
                        has_previous_page,
                        has_next_page,
                        start_cursor: Some(start_cursor.encode_cursor()),
                        end_cursor: Some(end_cursor.encode_cursor()),
                    },
                };
                Ok(connection)
            },
        )
        .await
    }
}

#[derive(sqlx::FromRow, SimpleObject)]
struct Post {
    id: u32,
    title: String,
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data::<MyToken>()?.0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}

pub fn get_role(session: &Arc<RwLock<Session>>) -> Option<Role> {
    let x: RwLockReadGuard<Session> = session.read().unwrap();
    let nn = x.get::<String>("user");

    Some(Role::Guest)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE TABLE posts(id INTEGER PRIMARY KEY, title TEXT);")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('ester');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('cookie');")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO posts(title) VALUES('cookie');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('cookie');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('dave');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('bosco');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('frida');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO posts(title) VALUES('giggle');")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO posts(title) VALUES('jasmine');")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO posts(title) VALUES('jerry');")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO posts(title) VALUES('alice');")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO posts(title) VALUES('iggy');")
        .execute(&pool)
        .await
        .unwrap();

    let session_store = RedisSessionStore::new("redis://127.0.0.1:6379/").unwrap();
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot).finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = warp::post()
        .map(move || pool.clone())
        .and(warp::cookie::optional::<String>("sid"))
        .and(async_graphql_warp::graphql(schema.clone()))
        .and(warp_sessions::request::with_session(
            session_store,
            Some(CookieOptions {
                http_only: true,
                cookie_name: "sid",
                secure: false,
                cookie_value: None,
                domain: Some("localhost".to_string()),
                max_age: Some(600),
                path: Some("/".to_string()),
                same_site: Some(SameSiteCookieOption::Strict),
            }),
        ))
        .and_then(
            |pool,
             sid: Option<String>,
             (schema, mut request): (
                Schema<QueryRoot, EmptyMutation, SubscriptionRoot>,
                async_graphql::Request,
            ),
             mut session_with_store: SessionWithStore<RedisSessionStore>| async move {
                let shared_session = Arc::new(RwLock::new(session_with_store.session));
                let maybe_role = get_role(&shared_session);
                if let Some(role) = maybe_role {
                    request = request.data(role);
                }
                request = request.data(shared_session.clone()).data(sid);
                request = request.data(pool);
                let resp = schema.execute(request).await;
                session_with_store.session = Arc::try_unwrap(shared_session)
                    .unwrap()
                    .into_inner()
                    .unwrap();
                Ok::<_, Infallible>((Response::from(resp), session_with_store))
            },
        )
        .untuple_one()
        .and_then(warp_sessions::reply::with_session);

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
            ))
    });

    let routes = graphql_subscription_with_data(schema, |value| async {
        #[derive(serde_derive::Deserialize)]
        struct Payload {
            token: String,
        }

        if let Ok(payload) = serde_json::from_value::<Payload>(value) {
            let mut data = Data::default();
            data.insert(MyToken(payload.token));
            Ok(data)
        } else {
            Err("Token is required".into())
        }
    })
    .or(graphql_playground)
    .or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
