//! Contains types related to the [info
//! field](https://www.asyncapi.com/docs/concepts/asyncapi-document/structure#info-field)
use crate::spec::common::{ExternalDocumentation, RefOr, Tag};

/// The info field in an API document offers crucial metadata, including the API's title,
/// version, description, contact details, and license. This field provides a
/// comprehensive overview of the API, aiding developers, architects, and other
/// stakeholders in quickly grasping its purpose and capabilities. As a mandatory element
/// of the AsyncAPI specification, the info field often serves as the initial reference
/// point for users navigating the API documentation.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The title of the application.
    pub title: String,
    /// Provides the version of the application API (not to be confused with the specification version).
    pub version: String,
    /// A short description of the application. CommonMark syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API. This MUST be in the form of an absolute URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    /// A list of tags for application API documentation control. Tags can be used for logical grouping of applications.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Additional external documentation of the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information. This MUST be in the form of an absolute URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization. MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. This MUST be in the form of an absolute URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
