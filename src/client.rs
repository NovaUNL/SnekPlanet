use iron_planet;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::sync::Arc;

use crate::models;

#[pyclass]
pub(crate) struct Supernova {
    config: iron_planet::RequestConfig,
    client: Arc<iron_planet::Supernova>,
}

#[pymethods]
impl Supernova {
    #[new]
    fn new() -> Self {
        Supernova {
            client: iron_planet::Supernova::new(),
            config: Default::default(),
        }
    }

    fn login(&self, username: &str, password: &str) -> PyResult<String> {
        match self.client.login(username, password) {
            Ok(val) => PyResult::Ok(val),
            Err(e) => PyResult::Err(exceptions::PyException::new_err(format!(
                "Login failed: {:?}",
                e
            ))),
        }
    }

    fn set_auth_token(&self, token: String) -> PyResult<()> {
        match self.client.set_auth_token(token) {
            Ok(val) => PyResult::Ok(val),
            Err(_) => PyResult::Err(exceptions::PyException::new_err(
                "Failed to set the auth token",
            )),
        }
    }

    fn logout(&self) -> PyResult<()> {
        match self.client.logout() {
            Ok(_) => PyResult::Ok(()),
            Err(e) => PyResult::Err(exceptions::PyException::new_err(format!(
                "Failed to logout: {:?}",
                e
            ))),
        }
    }

    #[getter]
    fn departments(&self) -> PyResult<Vec<models::Department>> {
        Ok(self
            .client
            .get_departments(&self.config)
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|department| models::Department { department })
            .collect())
    }

    #[getter]
    fn buildings(&self) -> PyResult<Vec<models::Building>> {
        Ok(self
            .client
            .get_buildings(&self.config)
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|building| models::Building { building })
            .collect())
    }

    #[getter]
    fn places(&self) -> PyResult<Vec<models::Place>> {
        Ok(self
            .client
            .get_places(&self.config)
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|place| models::Place { place })
            .collect())
    }

    #[getter]
    fn courses(&self) -> PyResult<Vec<models::Course>> {
        Ok(self
            .client
            .get_courses(&self.config)
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|course| models::Course { course })
            .collect())
    }
}
