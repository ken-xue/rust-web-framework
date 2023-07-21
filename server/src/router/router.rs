// use async_graphql::futures_util::SinkExt;
// use axum::http::StatusCode;
// use axum::response::IntoResponse;
// use axum::{Json, Router};
// use axum::routing::get;
// use axum::routing::post;
//
// fn initialize() -> Router<> {
//     // build our application with a route
//     return  Router::new()
//         .with()
//         // `GET /` goes to `root`
//         .route("/", get(root))
//         // `POST /users` goes to `create_user`
//         // .route("/users", post(create_user));
// }
// async fn root() -> &'static str {
//     "Hello, World!"
// }
//
// //
// // async fn create_user(
// //     // this argument tells axum to parse the request body
// //     // as JSON into a `AddUser` type
// //     Json(payload): Json<AddUser>,
// // ) -> impl IntoResponse {
// //     // insert your application logic here
// //     let user = User {
// //         id: 1337,
// //         username: payload.username,
// //     };
// //
// //     // this will be converted into a JSON response
// //     // with a status code of `201 Created`
// //     (StatusCode::CREATED, Json(user))
// // }
// //
// // // the input to our `create_user` handler
// // #[derive(Deserialize)]
// // struct AddUser {
// //     username: String,
// // }
// //
// // // the output to our `create_user` handler
// // #[derive(Serialize)]
// // struct User {
// //     id: u64,
// //     username: String,
// // }