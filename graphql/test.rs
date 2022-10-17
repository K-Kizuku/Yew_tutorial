#![allow(clippy::all, warnings)]
pub struct test;
pub mod test {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "test";
    pub const QUERY: &str =
        "query test {\n  todos{\n    user{\n      name\n    }\n    text\n    id\n    done\n  }\n}";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub todos: Vec<TestTodos>,
    }
    #[derive(Deserialize)]
    pub struct TestTodos {
        pub user: TestTodosUser,
        pub text: String,
        pub id: ID,
        pub done: Boolean,
    }
    #[derive(Deserialize)]
    pub struct TestTodosUser {
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for test {
    type Variables = test::Variables;
    type ResponseData = test::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: test::QUERY,
            operation_name: test::OPERATION_NAME,
        }
    }
}
