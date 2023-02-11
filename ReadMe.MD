# How to fix error[E0433]: failed to resolve: use of undeclared crate or module in rust documentation

Let's take a calculator project for example,
the Cargo.toml file looks like this.
```
[package]
name = "blog"
version = "0.1.0"
edition = "2021"
```
And the project folder appears like this
calculator
|__src
    |__main.rs
    |__utils.rs
    |__utils
        |__post.rs
when documenting in rust, crates, modules, enums, structs and functions should be,
treated as external package not as crate root.
>utils.rs looks like this
> ```
>   pub mod post;
>```
    ```
    //operator.rs
    pub struct Author{
    /// # Example snipet to display Author details
    /// **Note this line**
    /// //the crate was treated like an external package with this syntax
    /// // use project_name::crate::module::*;
    /// use blog::utils::post::*;
    /// //we can now use the struct or any part of the crate in the doc
    /// let sport_author = Author{first_name: "jhon".to_string(),
    /// last_name: "Doe".to_string(), email: "jhon.doe@example.com".to_string()}
    ///println!("{}", sport_author.display());
     pub first_name: String,
    pub last_name: String,
    pub email: String,
    } 
    impl Author{
    pub fn display(&self)->String{
    format!("First Name: {} \n Last Name: {} \n Email: {}", self.last_name, self.last_name, self.email);
    }
    }
    ```
