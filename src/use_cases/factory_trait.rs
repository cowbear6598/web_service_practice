use mongodb::Client;

pub trait UseCaseFactory {
    type Repo;
    type UseCase;

    fn new_repo(client: &Client) -> Self::Repo;
    fn new_use_case(repo: Self::Repo) -> Self::UseCase;
}

pub fn get_use_case<T: UseCaseFactory>(client: &Client) -> T::UseCase {
    let repo = T::new_repo(client);
    let use_case = T::new_use_case(repo);

    use_case
}