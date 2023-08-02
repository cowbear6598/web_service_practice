use mongodb::Client;

pub trait ServiceFactoryTrait {
    type Repository;
    type UseCase;
    type Service;

    fn new_repo(client: &Client) -> Self::Repository;
    fn new_use_case(repo: Self::Repository) -> Self::UseCase;
    fn new_service(use_case: Self::UseCase) -> Self::Service;
}

pub fn get_service<T: ServiceFactoryTrait>(client: &Client) -> T::Service {
    let repo = T::new_repo(client);
    let use_case = T::new_use_case(repo);
    let service = T::new_service(use_case);

    service
}