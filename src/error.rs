use std::{error::Error, fmt::Display};

use crate::mochow::{api::*, client::*, config::*};

#[derive(Debug)]
pub enum SdkError {
    /// Request error.
    RequestError(reqwest::Error),

    /// Request middleware error.
    RequestMiddlewareError(reqwest_middleware::Error),

    /// Mochow service error. with service code, message and http status code.
    ServiceError(ServiceError),

    /// Mochow SDK param error.
    ParamsError(String),

    /// Other error.
    OtherError(anyhow::Error),
}

impl Error for SdkError {}

impl Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdkError::ServiceError(e) => write!(f, "service Error: {}", e),
            SdkError::OtherError(e) => write!(f, "other error: {}. \ndetail {:?}", e, e),
            SdkError::ParamsError(e) => write!(f, "params error: {}", e),
            SdkError::RequestError(e) => {
                write!(f, "request error: {}. \ndetail {:?}", e, e)
            }
            SdkError::RequestMiddlewareError(e) => {
                write!(f, "request middleware error: {}. \ndetail {:?}", e, e)
            }
        }
    }
}

impl From<reqwest::Error> for SdkError {
    fn from(value: reqwest::Error) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<reqwest_middleware::Error> for SdkError {
    fn from(value: reqwest_middleware::Error) -> Self {
        SdkError::RequestMiddlewareError(value)
    }
}

impl From<ClientConfigurationBuilderError> for SdkError {
    fn from(value: ClientConfigurationBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<MochowClientBuilderError> for SdkError {
    fn from(value: MochowClientBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<CreateDatabaseArgsBuilderError> for SdkError {
    fn from(value: CreateDatabaseArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<DropDatabaseArgsBuilderError> for SdkError {
    fn from(value: DropDatabaseArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<ListDatabaseArgsBuilderError> for SdkError {
    fn from(value: ListDatabaseArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<DropTableArgsBuilderError> for SdkError {
    fn from(value: DropTableArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<ListTableArgsBuilderError> for SdkError {
    fn from(value: ListTableArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<DescriptTableArgsBuilderError> for SdkError {
    fn from(value: DescriptTableArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<StatsTableArgsBuilderError> for SdkError {
    fn from(value: StatsTableArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<DescriptIndexArgsBuilderError> for SdkError {
    fn from(value: DescriptIndexArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<RebuildIndexArgsBuilderError> for SdkError {
    fn from(value: RebuildIndexArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}

impl From<DeleteIndexArgsBuilderError> for SdkError {
    fn from(value: DeleteIndexArgsBuilderError) -> Self {
        SdkError::OtherError(value.into())
    }
}
