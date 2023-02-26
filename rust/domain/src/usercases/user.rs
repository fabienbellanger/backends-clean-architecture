//! User use cases

/// Create new user use case
struct CreateUser {
    user_repository: String, // TODO: change by UserRepository
}

impl CreateUser {
    /// Create a new `CreateUser` type
    fn new(repo: String) -> Self { // TODO: change by UserRepository
        Self { user_repository: repo }
    }
}
