pub trait ApiService {
    fn fetch_data(&self) -> Result<String, String>;
}

pub struct MyApp<T> {
    service: T
}

impl <T: ApiService> MyApp<T> {
    pub fn new(service: T) -> Self{
        MyApp {
            service
        }
    }

    pub fn get_data(&self) -> Result<String, String> {
        self.service.fetch_data()
    }
}

#[cfg(test)]
mod tests{
    use crate::learning_series::serie2_trait_mock::*;

    struct MockApiService {
        return_value: Result<String, String>,
    }

    impl MockApiService {
        fn new(return_value: Result<String, String>) -> Self {
            Self {
                return_value
            }
        }
    }

    impl ApiService for MockApiService {
        fn fetch_data(&self) -> Result<String, String> {
            self.return_value.clone()
        }
    }

    #[test]
    fn fetch_data_sucess() {
        let mock_service = MockApiService::new(Ok("data".to_string()));
        let app = MyApp::new(mock_service);
        let result = app.get_data();

        assert_eq!(result.unwrap(), "data");
    }
}

