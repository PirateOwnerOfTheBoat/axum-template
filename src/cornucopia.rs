// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod users
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,> { pub email: T1,pub username: T2,pub password_hash: T3,}#[derive( Debug, Clone, PartialEq,)] pub struct FindByUsername
{ pub id : i64,pub email : String,pub username : String,pub password_hash : String,}pub struct FindByUsernameBorrowed<'a> { pub id : i64,pub email : &'a str,pub username : &'a str,pub password_hash : &'a str,}
impl<'a> From<FindByUsernameBorrowed<'a>> for FindByUsername
{
    fn from(FindByUsernameBorrowed { id,email,username,password_hash,}: FindByUsernameBorrowed<'a>) ->
    Self { Self { id,email: email.into(),username: username.into(),password_hash: password_hash.into(),} }
}pub struct FindByUsernameQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> FindByUsernameBorrowed,
    mapper: fn(FindByUsernameBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> FindByUsernameQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(FindByUsernameBorrowed) -> R) ->
    FindByUsernameQuery<'a,C,R,N>
    {
        FindByUsernameQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct FindById
{ pub id : i64,pub email : String,pub username : String,pub password_hash : String,}pub struct FindByIdBorrowed<'a> { pub id : i64,pub email : &'a str,pub username : &'a str,pub password_hash : &'a str,}
impl<'a> From<FindByIdBorrowed<'a>> for FindById
{
    fn from(FindByIdBorrowed { id,email,username,password_hash,}: FindByIdBorrowed<'a>) ->
    Self { Self { id,email: email.into(),username: username.into(),password_hash: password_hash.into(),} }
}pub struct FindByIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> FindByIdBorrowed,
    mapper: fn(FindByIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> FindByIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(FindByIdBorrowed) -> R) ->
    FindByIdQuery<'a,C,R,N>
    {
        FindByIdQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn find_by_username() -> FindByUsernameStmt
{ FindByUsernameStmt(cornucopia_async::private::Stmt::new("SELECT
    id,
    email,
    username,
    password_hash
FROM users
WHERE username = $1")) } pub struct
FindByUsernameStmt(cornucopia_async::private::Stmt); impl FindByUsernameStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
username: &'a T1,) -> FindByUsernameQuery<'a,C,
FindByUsername, 1>
{
    FindByUsernameQuery
    {
        client, params: [username,], stmt: &mut self.0, extractor:
        |row| { FindByUsernameBorrowed { id: row.get(0),email: row.get(1),username: row.get(2),password_hash: row.get(3),} }, mapper: |it| { <FindByUsername>::from(it) },
    }
} }pub fn find_by_id() -> FindByIdStmt
{ FindByIdStmt(cornucopia_async::private::Stmt::new("SELECT
    id,
    email,
    username,
    password_hash
FROM users
WHERE id = $1")) } pub struct
FindByIdStmt(cornucopia_async::private::Stmt); impl FindByIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i64,) -> FindByIdQuery<'a,C,
FindById, 1>
{
    FindByIdQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { FindByIdBorrowed { id: row.get(0),email: row.get(1),username: row.get(2),password_hash: row.get(3),} }, mapper: |it| { <FindById>::from(it) },
    }
} }pub fn insert() -> InsertStmt
{ InsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO users(email, username, password_hash)
VALUES ($1, $2, $3)")) } pub struct
InsertStmt(cornucopia_async::private::Stmt); impl InsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
email: &'a T1,username: &'a T2,password_hash: &'a T3,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[email,username,password_hash,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, InsertParams<T1,T2,T3,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for InsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertParams<T1,T2,T3,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.email,&params.username,&params.password_hash,)) }
}}}