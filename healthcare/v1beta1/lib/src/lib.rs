pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(rename = "auditLogConfigs", default)]
        pub audit_log_configs: Option<Vec<crate::schemas::AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging.\nFor example, `storage.googleapis.com`, `cloudsql.googleapis.com`.\n`allServices` is a special value that covers all services."]
        #[serde(rename = "service", default)]
        pub service: Option<String>,
    }
    impl ::field_selector::FieldSelector for AuditConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuditLogConfigLogType {
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
    }
    impl AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
                AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                AuditLogConfigLogType::DataRead => "DATA_READ",
            }
        }
    }
    impl ::std::fmt::Display for AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of\npermission.\nFollows the same format of Binding.members."]
        #[serde(rename = "exemptedMembers", default)]
        pub exempted_members: Option<Vec<String>>,
        #[doc = "Specifies whether principals can be exempted for the same LogType in\nlower-level resource policies. If true, any lower-level exemptions will\nbe ignored."]
        #[serde(rename = "ignoreChildExemptions", default)]
        pub ignore_child_exemptions: Option<bool>,
        #[doc = "The log type that this config enables."]
        #[serde(rename = "logType", default)]
        pub log_type: Option<crate::schemas::AuditLogConfigLogType>,
    }
    impl ::field_selector::FieldSelector for AuditLogConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Binding {
        #[doc = "The condition that is associated with this binding.\nNOTE: An unsatisfied condition will not allow user access via current\nbinding. Different bindings, including their conditions, are examined\nindependently."]
        #[serde(rename = "condition", default)]
        pub condition: Option<crate::schemas::Expr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource.\n`members` can have the following values:\n\n* `allUsers`: A special identifier that represents anyone who is\n   on the internet; with or without a Google account.\n\n* `allAuthenticatedUsers`: A special identifier that represents anyone\n   who is authenticated with a Google account or a service account.\n\n* `user:{emailid}`: An email address that represents a specific Google\n   account. For example, `alice@example.com` .\n\n\n* `serviceAccount:{emailid}`: An email address that represents a service\n   account. For example, `my-other-app@appspot.gserviceaccount.com`.\n\n* `group:{emailid}`: An email address that represents a Google group.\n   For example, `admins@example.com`.\n\n\n* `domain:{domain}`: The G Suite domain (primary) that represents all the\n   users of that domain. For example, `google.com` or `example.com`.\n\n"]
        #[serde(rename = "members", default)]
        pub members: Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`.\nFor example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
    }
    impl ::field_selector::FieldSelector for Binding {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CharacterMaskConfig {
        #[doc = "Character to mask the sensitive values. If not supplied, defaults to \"*\"."]
        #[serde(rename = "maskingCharacter", default)]
        pub masking_character: Option<String>,
    }
    impl ::field_selector::FieldSelector for CharacterMaskConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateMessageRequest {
        #[doc = "HL7v2 message."]
        #[serde(rename = "message", default)]
        pub message: Option<crate::schemas::Message>,
    }
    impl ::field_selector::FieldSelector for CreateMessageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CryptoHashConfig {
        #[doc = "An AES 128/192/256 bit key. Causes the hash to be computed based on this\nkey. A default key is generated for each DeidentifyDataset operation and is\nused wherever crypto_key is not specified."]
        #[serde(rename = "cryptoKey", default)]
        pub crypto_key: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for CryptoHashConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Dataset {
        #[doc = "Output only. Resource name of the dataset, of the form\n`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The default timezone used by this dataset. Must be a either a valid IANA\ntime zone name such as \"America/New_York\" or empty, which defaults to UTC.\nThis is used for parsing times in resources (e.g., HL7 messages) where no\nexplicit timezone is specified."]
        #[serde(rename = "timeZone", default)]
        pub time_zone: Option<String>,
    }
    impl ::field_selector::FieldSelector for Dataset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DateShiftConfig {
        #[doc = "An AES 128/192/256 bit key. Causes the shift to be computed based on this\nkey and the patient ID. A default key is generated for each\nDeidentifyDataset operation and is used wherever crypto_key is not\nspecified."]
        #[serde(rename = "cryptoKey", default)]
        pub crypto_key: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for DateShiftConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeidentifyConfig {
        #[doc = "Configures de-id of application/DICOM content."]
        #[serde(rename = "dicom", default)]
        pub dicom: Option<crate::schemas::DicomConfig>,
        #[doc = "Configures de-id of application/FHIR content."]
        #[serde(rename = "fhir", default)]
        pub fhir: Option<crate::schemas::FhirConfig>,
        #[doc = "Configures de-identification of image pixels wherever they are found in the\nsource_dataset."]
        #[serde(rename = "image", default)]
        pub image: Option<crate::schemas::ImageConfig>,
        #[doc = "Configures de-identification of text wherever it is found in the\nsource_dataset."]
        #[serde(rename = "text", default)]
        pub text: Option<crate::schemas::TextConfig>,
    }
    impl ::field_selector::FieldSelector for DeidentifyConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeidentifyDatasetRequest {
        #[doc = "Deidentify configuration."]
        #[serde(rename = "config", default)]
        pub config: Option<crate::schemas::DeidentifyConfig>,
        #[doc = "The name of the dataset resource to create and write the redacted data to\n(e.g.,\n\n * The destination dataset must not exist.\n * The destination dataset must be in the same project as the source\n   dataset. De-identifying data across multiple projects is not supported."]
        #[serde(rename = "destinationDataset", default)]
        pub destination_dataset: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeidentifyDatasetRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeidentifyErrorDetails {
        #[doc = "Number of resources failed to process."]
        #[serde(rename = "failureResourceCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub failure_resource_count: Option<i64>,
        #[doc = "Number of stores failed to process."]
        #[serde(rename = "failureStoreCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub failure_store_count: Option<i64>,
        #[doc = "Number of resources successfully processed."]
        #[serde(rename = "successResourceCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub success_resource_count: Option<i64>,
        #[doc = "Number of stores successfully processed."]
        #[serde(rename = "successStoreCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub success_store_count: Option<i64>,
    }
    impl ::field_selector::FieldSelector for DeidentifyErrorDetails {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeidentifySummary {
        #[doc = "Number of resources successfully processed."]
        #[serde(rename = "successResourceCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub success_resource_count: Option<i64>,
        #[doc = "Number of stores successfully processed."]
        #[serde(rename = "successStoreCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub success_store_count: Option<i64>,
    }
    impl ::field_selector::FieldSelector for DeidentifySummary {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DicomConfigFilterProfile {
        #[doc = "No tag filtration profile provided. Same as KEEP_ALL_PROFILE."]
        TagFilterProfileUnspecified,
        #[doc = "Keep only tags required to produce valid DICOM."]
        MinimalKeepListProfile,
        #[doc = "Remove tags based on DICOM Standard's Attribute Confidentiality Basic\nProfile (DICOM Standard Edition 2018e)\nhttp://dicom.nema.org/medical/dicom/2018e/output/chtml/part15/chapter_E.html."]
        AttributeConfidentialityBasicProfile,
        #[doc = "Keep all tags."]
        KeepAllProfile,
        #[doc = "Inspects within tag contents and replaces sensitive text. The process\ncan be configured using the TextConfig.\nApplies to all tags with the following Value Representation names:\nAE, LO, LT, PN, SH, ST, UC, UT, DA, DT, AS"]
        DeidentifyTagContents,
    }
    impl DicomConfigFilterProfile {
        pub fn as_str(self) -> &'static str {
            match self {
                DicomConfigFilterProfile::TagFilterProfileUnspecified => {
                    "TAG_FILTER_PROFILE_UNSPECIFIED"
                }
                DicomConfigFilterProfile::MinimalKeepListProfile => "MINIMAL_KEEP_LIST_PROFILE",
                DicomConfigFilterProfile::AttributeConfidentialityBasicProfile => {
                    "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE"
                }
                DicomConfigFilterProfile::KeepAllProfile => "KEEP_ALL_PROFILE",
                DicomConfigFilterProfile::DeidentifyTagContents => "DEIDENTIFY_TAG_CONTENTS",
            }
        }
    }
    impl ::std::fmt::Display for DicomConfigFilterProfile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DicomConfigFilterProfile {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DicomConfigFilterProfile {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TAG_FILTER_PROFILE_UNSPECIFIED" => {
                    DicomConfigFilterProfile::TagFilterProfileUnspecified
                }
                "MINIMAL_KEEP_LIST_PROFILE" => DicomConfigFilterProfile::MinimalKeepListProfile,
                "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE" => {
                    DicomConfigFilterProfile::AttributeConfidentialityBasicProfile
                }
                "KEEP_ALL_PROFILE" => DicomConfigFilterProfile::KeepAllProfile,
                "DEIDENTIFY_TAG_CONTENTS" => DicomConfigFilterProfile::DeidentifyTagContents,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DicomConfig {
        #[doc = "Tag filtering profile that determines which tags to keep/remove."]
        #[serde(rename = "filterProfile", default)]
        pub filter_profile: Option<crate::schemas::DicomConfigFilterProfile>,
        #[doc = "List of tags to keep. Remove all other tags."]
        #[serde(rename = "keepList", default)]
        pub keep_list: Option<crate::schemas::TagFilterList>,
        #[doc = "List of tags to remove. Keep all other tags."]
        #[serde(rename = "removeList", default)]
        pub remove_list: Option<crate::schemas::TagFilterList>,
    }
    impl ::field_selector::FieldSelector for DicomConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DicomStore {
        #[doc = "User-supplied key-value pairs used to organize DICOM stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding\nof maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression:\n\\p{Ll}\\p{Lo}{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have\na UTF-8 encoding of maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. Resource name of the DICOM store, of the form\n`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Notification destination for new DICOM instances.\nSupplied by the client."]
        #[serde(rename = "notificationConfig", default)]
        pub notification_config: Option<crate::schemas::NotificationConfig>,
    }
    impl ::field_selector::FieldSelector for DicomStore {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty;
    impl ::field_selector::FieldSelector for Empty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ErrorDetail {
        #[doc = "The status of the error."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::Status>,
        #[doc = "The identifier of the resource."]
        #[serde(rename = "resource", default)]
        pub resource: Option<String>,
    }
    impl ::field_selector::FieldSelector for ErrorDetail {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ExportDicomDataRequest {
        #[doc = "The BigQuery output destination.\n\nYou can only export to a BigQuery dataset that's in the same project as\nthe DICOM store you're exporting from.\n\nThe BigQuery location requires two IAM roles:\n`roles/bigquery.dataEditor` and `roles/bigquery.jobUser`."]
        #[serde(rename = "bigqueryDestination", default)]
        pub bigquery_destination:
            Option<crate::schemas::GoogleCloudHealthcareV1Beta1DicomBigQueryDestination>,
        #[doc = "The Cloud Storage output destination.\n\nThe Cloud Storage location requires the `roles/storage.objectAdmin` Cloud\nIAM role."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            Option<crate::schemas::GoogleCloudHealthcareV1Beta1DicomGcsDestination>,
    }
    impl ::field_selector::FieldSelector for ExportDicomDataRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ExportResourcesRequest {
        #[doc = "The BigQuery output destination.\n\nThe BigQuery location requires two IAM roles:\n`roles/bigquery.dataEditor` and `roles/bigquery.jobUser`.\n\nThe output will be one BigQuery table per resource type."]
        #[serde(rename = "bigqueryDestination", default)]
        pub bigquery_destination:
            Option<crate::schemas::GoogleCloudHealthcareV1Beta1FhirBigQueryDestination>,
        #[doc = "The Cloud Storage output destination.\n\nThe Cloud Storage location requires the `roles/storage.objectAdmin` Cloud\nIAM role.\n\nThe exported outputs are\norganized by FHIR resource types. The server will create one object per\nresource type. Each object contains newline delimited JSON, and each line\nis a FHIR resource."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            Option<crate::schemas::GoogleCloudHealthcareV1Beta1FhirRestGcsDestination>,
    }
    impl ::field_selector::FieldSelector for ExportResourcesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Expr {
        #[doc = "An optional description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Textual representation of an expression in\nCommon Expression Language syntax.\n\nThe application context of the containing message determines which\nwell-known feature set of CEL is supported."]
        #[serde(rename = "expression", default)]
        pub expression: Option<String>,
        #[doc = "An optional string indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "An optional title for the expression, i.e. a short string describing\nits purpose. This can be used e.g. in UIs which allow to enter the\nexpression."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Expr {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FhirConfig {
        #[doc = "Specifies FHIR paths to match and how to transform them. Any field that\nis not matched by a FieldMetadata will be passed through to the output\ndataset unmodified. All extensions are removed in the output."]
        #[serde(rename = "fieldMetadataList", default)]
        pub field_metadata_list: Option<Vec<crate::schemas::FieldMetadata>>,
    }
    impl ::field_selector::FieldSelector for FhirConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FhirStore {
        #[doc = "Whether to disable referential integrity in this FHIR store. This field is\nimmutable after FHIR store creation.\nThe default value is false, meaning that the API will enforce referential\nintegrity and fail the requests that will result in inconsistent state in\nthe FHIR store.\nWhen this field is set to true, the API will skip referential integrity\ncheck. Consequently, operations that rely on references, such as\nGetPatientEverything, will not return all the results if broken references\nexist."]
        #[serde(rename = "disableReferentialIntegrity", default)]
        pub disable_referential_integrity: Option<bool>,
        #[doc = "Whether to disable resource versioning for this FHIR store. This field can\nnot be changed after the creation of FHIR store.\nIf set to false, which is the default behavior, all write operations will\ncause historical versions to be recorded automatically. The historical\nversions can be fetched through the history APIs, but cannot be updated.\nIf set to true, no historical versions will be kept. The server will send\nback errors for attempts to read the historical versions."]
        #[serde(rename = "disableResourceVersioning", default)]
        pub disable_resource_versioning: Option<bool>,
        #[doc = "Whether to allow the bulk import API to accept history bundles and directly\ninsert historical resource versions into the FHIR store. Importing resource\nhistories creates resource interactions that appear to have occurred in the\npast, which clients may not want to allow. If set to false, history bundles\nwithin an import will fail with an error."]
        #[serde(rename = "enableHistoryImport", default)]
        pub enable_history_import: Option<bool>,
        #[doc = "Whether this FHIR store has the [updateCreate\ncapability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate).\nThis determines if the client can use an Update operation to create a new\nresource with a client-specified ID. If false, all IDs are server-assigned\nthrough the Create operation and attempts to Update a non-existent resource\nwill return errors. Please treat the audit logs with appropriate levels of\ncare if client-specified resource IDs contain sensitive data such as\npatient identifiers, those IDs will be part of the FHIR resource path\nrecorded in Cloud audit logs and Cloud Pub/Sub notifications."]
        #[serde(rename = "enableUpdateCreate", default)]
        pub enable_update_create: Option<bool>,
        #[doc = "User-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding\nof maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression:\n\\p{Ll}\\p{Lo}{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have\na UTF-8 encoding of maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. Resource name of the FHIR store, of the form\n`projects/{project_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "If non-empty, publish all resource modifications of this FHIR store to\nthis destination. The Cloud Pub/Sub message attributes will contain a map\nwith a string describing the action that has triggered the notification,\ne.g. \"action\":\"CreateResource\"."]
        #[serde(rename = "notificationConfig", default)]
        pub notification_config: Option<crate::schemas::NotificationConfig>,
    }
    impl ::field_selector::FieldSelector for FhirStore {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldMetadataAction {
        #[doc = "No action specified."]
        ActionUnspecified,
        #[doc = "Transform the entire field."]
        Transform,
        #[doc = "Inspect and transform any found PHI."]
        InspectAndTransform,
        #[doc = "Do not transform."]
        DoNotTransform,
    }
    impl FieldMetadataAction {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldMetadataAction::ActionUnspecified => "ACTION_UNSPECIFIED",
                FieldMetadataAction::Transform => "TRANSFORM",
                FieldMetadataAction::InspectAndTransform => "INSPECT_AND_TRANSFORM",
                FieldMetadataAction::DoNotTransform => "DO_NOT_TRANSFORM",
            }
        }
    }
    impl ::std::fmt::Display for FieldMetadataAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldMetadataAction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldMetadataAction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_UNSPECIFIED" => FieldMetadataAction::ActionUnspecified,
                "TRANSFORM" => FieldMetadataAction::Transform,
                "INSPECT_AND_TRANSFORM" => FieldMetadataAction::InspectAndTransform,
                "DO_NOT_TRANSFORM" => FieldMetadataAction::DoNotTransform,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FieldMetadata {
        #[doc = "Deidentify action for one field."]
        #[serde(rename = "action", default)]
        pub action: Option<crate::schemas::FieldMetadataAction>,
        #[doc = "List of paths to FHIR fields to be redacted. Each path is a\nperiod-separated list where each component is either a field name or\nFHIR type name, for example: Patient, HumanName.\nFor \"choice\" types (those defined in the FHIR spec with the form:\nfield[x]) we use two separate components. e.g. \"deceasedAge.unit\" is\nmatched by \"Deceased.Age.unit\".\nSupported types are: AdministrativeGenderCode, Code, Date, DateTime,\nDecimal, HumanName, Id, LanguageCode, Markdown, MimeTypeCode, Oid,\nString, Uri, Uuid, Xhtml."]
        #[serde(rename = "paths", default)]
        pub paths: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for FieldMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1DicomBigQueryDestination {
        #[doc = "If the destination table already exists and this flag is `TRUE`, the table\nwill be overwritten by the contents of the DICOM store. If the flag is not\nset and the destination table already exists, the export call returns an\nerror."]
        #[serde(rename = "force", default)]
        pub force: Option<bool>,
        #[doc = "BigQuery URI to a table, up to 2000 characters long, in the format\n`bq://projectId.bqDatasetId.tableId`"]
        #[serde(rename = "tableUri", default)]
        pub table_uri: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudHealthcareV1Beta1DicomBigQueryDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1DicomGcsDestination {
        #[doc = "MIME types supported by DICOM spec.\nEach file will be written in the following format:\n`.../{study_id}/{series_id}/{instance_id}[/{frame_number}].{extension}`\nThe frame_number component will exist only for multi-frame instances.\n\nRefer to the DICOM conformance statement for permissible MIME types:\nhttps://cloud.google.com/healthcare/docs/dicom#wado-rs\n\nThe following extensions will be used for output files:\n  application/dicom -> .dcm\n  image/jpeg -> .jpg\n  image/png -> .png\n\nIf unspecified, the instances will be exported in their original\nDICOM format."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
        #[doc = "The Cloud Storage destination to export to.\n\nURI for a Cloud Storage directory where result files should be written (in\nthe format `gs://{bucket-id}/{path/to/destination/dir}`). If there is no\ntrailing slash, the service will append one when composing the object path.\nThe user is responsible for creating the Cloud Storage bucket referenced in\n`uri_prefix`."]
        #[serde(rename = "uriPrefix", default)]
        pub uri_prefix: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudHealthcareV1Beta1DicomGcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1DicomGcsSource {
        #[doc = "Points to a Cloud Storage URI containing file(s) with\ncontent only. The URI must be in the following format:\n`gs://{bucket_id}/{object_id}`. The URI can include wildcards in\n`object_id` and thus identify multiple files. Supported wildcards:\n '*' to match 0 or more non-separator characters\n '**' to match 0 or more characters (including separators). Must be used at\n      the end of a path and with no other wildcards in the\n      path. Can also be used with a file extension (such as .dcm), which\n      imports all files with the extension in the specified directory and\n      its sub-directories. For example,\n      `gs://my-bucket/my-directory/**.dcm` imports all files with .dcm\n      extensions in `my-directory/` and its sub-directories.\n '?' to match 1 character\nAll other URI formats are invalid.\nFiles matching the wildcard are expected to contain content only, no\nmetadata."]
        #[serde(rename = "uri", default)]
        pub uri: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudHealthcareV1Beta1DicomGcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1FhirBigQueryDestination {
        #[doc = "BigQuery URI to a dataset, up to 2000 characters long, in the format\n`bq://projectId.bqDatasetId`"]
        #[serde(rename = "datasetUri", default)]
        pub dataset_uri: Option<String>,
        #[doc = "The configuration for the exported BigQuery schema."]
        #[serde(rename = "schemaConfig", default)]
        pub schema_config: Option<crate::schemas::SchemaConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudHealthcareV1Beta1FhirBigQueryDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1FhirRestExportResourcesResponse {
        #[doc = "The name of the FHIR store where resources have been exported, in the\nformat\n`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`."]
        #[serde(rename = "fhirStore", default)]
        pub fhir_store: Option<String>,
        #[doc = "The total number of resources exported from the requested FHIR store."]
        #[serde(rename = "resourceCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub resource_count: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudHealthcareV1Beta1FhirRestExportResourcesResponse
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1FhirRestGcsDestination {
        #[doc = "URI for a Cloud Storage directory where result files should be written (in\nthe format `gs://{bucket-id}/{path/to/destination/dir}`). If there is no\ntrailing slash, the service will append one when composing the object path.\nThe user is responsible for creating the Cloud Storage bucket referenced in\n`uri_prefix`."]
        #[serde(rename = "uriPrefix", default)]
        pub uri_prefix: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudHealthcareV1Beta1FhirRestGcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1FhirRestGcsSource {
        #[doc = "Points to a Cloud Storage URI containing file(s) to import.\n\nThe URI must be in the following format: `gs://{bucket_id}/{object_id}`.\nThe URI can include wildcards in `object_id` and thus identify multiple\nfiles. Supported wildcards:\n\n*  `*` to match 0 or more non-separator characters\n*  `**` to match 0 or more characters (including separators). Must be used\nat the end of a path and with no other wildcards in the\npath. Can also be used with a file extension (such as .ndjson), which\nimports all files with the extension in the specified directory and\nits sub-directories. For example, `gs://my-bucket/my-directory/**.ndjson`\nimports all files with `.ndjson` extensions in `my-directory/` and its\nsub-directories.\n*  `?` to match 1 character\n\nFiles matching the wildcard are expected to contain content only, no\nmetadata."]
        #[serde(rename = "uri", default)]
        pub uri: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudHealthcareV1Beta1FhirRestGcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1FhirRestImportResourcesErrorDetails {
        #[doc = "The number of resources that had errors."]
        #[serde(rename = "errorCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub error_count: Option<i64>,
        #[doc = "The name of the FHIR store where resources have been imported, in the\nformat\n`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`."]
        #[serde(rename = "fhirStore", default)]
        pub fhir_store: Option<String>,
        #[doc = "The total number of resources included in the source data. This is the sum\nof the success and error counts."]
        #[serde(rename = "inputSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub input_size: Option<i64>,
        #[doc = "The number of resources that have been imported."]
        #[serde(rename = "successCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub success_count: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudHealthcareV1Beta1FhirRestImportResourcesErrorDetails
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudHealthcareV1Beta1FhirRestImportResourcesResponse {
        #[doc = "The name of the FHIR store where the resources have been imported, in the\nformat\n`projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`."]
        #[serde(rename = "fhirStore", default)]
        pub fhir_store: Option<String>,
        #[doc = "The total number of resources included in the source data."]
        #[serde(rename = "inputSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub input_size: Option<i64>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudHealthcareV1Beta1FhirRestImportResourcesResponse
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Hl7V2Store {
        #[doc = "User-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding\nof maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression:\n\\p{Ll}\\p{Lo}{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have\na UTF-8 encoding of maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. Resource name of the HL7v2 store, of the form\n`projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The notification destination all messages (both Ingest & Create) are\npublished on. Only the message name is sent as part of the notification. If\nthis is unset, no notifications will be sent. Supplied by the client."]
        #[serde(rename = "notificationConfig", default)]
        pub notification_config: Option<crate::schemas::NotificationConfig>,
        #[doc = "The configuration for the parser. It determines how the server parses the\nmessages."]
        #[serde(rename = "parserConfig", default)]
        pub parser_config: Option<crate::schemas::ParserConfig>,
    }
    impl ::field_selector::FieldSelector for Hl7V2Store {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct HttpBody {
        #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
        #[serde(rename = "contentType", default)]
        pub content_type: Option<String>,
        #[doc = "The HTTP request/response body as raw binary."]
        #[serde(rename = "data", default)]
        pub data: Option<Vec<u8>>,
        #[doc = "Application specific response metadata. Must be set in the first response\nfor streaming APIs."]
        #[serde(rename = "extensions", default)]
        pub extensions: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::field_selector::FieldSelector for HttpBody {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageConfigTextRedactionMode {
        #[doc = "No text redaction specified. Same as REDACT_NO_TEXT."]
        TextRedactionModeUnspecified,
        #[doc = "Redact all text."]
        RedactAllText,
        #[doc = "Redact sensitive text."]
        RedactSensitiveText,
        #[doc = "Do not redact text."]
        RedactNoText,
    }
    impl ImageConfigTextRedactionMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageConfigTextRedactionMode::TextRedactionModeUnspecified => {
                    "TEXT_REDACTION_MODE_UNSPECIFIED"
                }
                ImageConfigTextRedactionMode::RedactAllText => "REDACT_ALL_TEXT",
                ImageConfigTextRedactionMode::RedactSensitiveText => "REDACT_SENSITIVE_TEXT",
                ImageConfigTextRedactionMode::RedactNoText => "REDACT_NO_TEXT",
            }
        }
    }
    impl ::std::fmt::Display for ImageConfigTextRedactionMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageConfigTextRedactionMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageConfigTextRedactionMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEXT_REDACTION_MODE_UNSPECIFIED" => {
                    ImageConfigTextRedactionMode::TextRedactionModeUnspecified
                }
                "REDACT_ALL_TEXT" => ImageConfigTextRedactionMode::RedactAllText,
                "REDACT_SENSITIVE_TEXT" => ImageConfigTextRedactionMode::RedactSensitiveText,
                "REDACT_NO_TEXT" => ImageConfigTextRedactionMode::RedactNoText,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImageConfig {
        #[doc = "Determines how to redact text from image."]
        #[serde(rename = "textRedactionMode", default)]
        pub text_redaction_mode: Option<crate::schemas::ImageConfigTextRedactionMode>,
    }
    impl ::field_selector::FieldSelector for ImageConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ImportDicomDataErrorDetails {
        #[doc = "Deprecated. Use only for debugging purposes.\n\nContains sample errors encountered in imports of individual resources\n(for example, a Cloud Storage object)."]
        #[serde(rename = "sampleErrors", default)]
        pub sample_errors: Option<Vec<crate::schemas::ErrorDetail>>,
    }
    impl ::field_selector::FieldSelector for ImportDicomDataErrorDetails {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImportDicomDataRequest {
        #[doc = "Cloud Storage source data location and import configuration.\n\nThe Cloud Storage location requires the `roles/storage.objectViewer`\nCloud IAM role."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: Option<crate::schemas::GoogleCloudHealthcareV1Beta1DicomGcsSource>,
    }
    impl ::field_selector::FieldSelector for ImportDicomDataRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImportResourcesRequestContentStructure {
        #[doc = "If the content structure is not specified, the default value `BUNDLE`\nwill be used."]
        ContentStructureUnspecified,
        #[doc = "The source file contains one or more lines of newline-delimited JSON\n(ndjson). Each line is a bundle, which contains one or more resources.\nSet the bundle type to `history` to import resource versions."]
        Bundle,
        #[doc = "The source file contains one or more lines of newline-delimited JSON\n(ndjson). Each line is a single resource."]
        Resource,
    }
    impl ImportResourcesRequestContentStructure {
        pub fn as_str(self) -> &'static str {
            match self {
                ImportResourcesRequestContentStructure::ContentStructureUnspecified => {
                    "CONTENT_STRUCTURE_UNSPECIFIED"
                }
                ImportResourcesRequestContentStructure::Bundle => "BUNDLE",
                ImportResourcesRequestContentStructure::Resource => "RESOURCE",
            }
        }
    }
    impl ::std::fmt::Display for ImportResourcesRequestContentStructure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImportResourcesRequestContentStructure {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImportResourcesRequestContentStructure {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_STRUCTURE_UNSPECIFIED" => {
                    ImportResourcesRequestContentStructure::ContentStructureUnspecified
                }
                "BUNDLE" => ImportResourcesRequestContentStructure::Bundle,
                "RESOURCE" => ImportResourcesRequestContentStructure::Resource,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImportResourcesRequest {
        #[doc = "The content structure in the source location. If not specified, the server\ntreats the input source files as BUNDLE."]
        #[serde(rename = "contentStructure", default)]
        pub content_structure: Option<crate::schemas::ImportResourcesRequestContentStructure>,
        #[doc = "Cloud Storage source data location and import configuration.\n\nThe Cloud Storage location requires the `roles/storage.objectViewer`\nCloud IAM role.\n\nEach Cloud Storage object should be a text file that contains the format\nspecified in ContentStructure."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: Option<crate::schemas::GoogleCloudHealthcareV1Beta1FhirRestGcsSource>,
    }
    impl ::field_selector::FieldSelector for ImportResourcesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InfoTypeTransformation {
        #[doc = "Config for character mask."]
        #[serde(rename = "characterMaskConfig", default)]
        pub character_mask_config: Option<crate::schemas::CharacterMaskConfig>,
        #[doc = "Config for crypto hash."]
        #[serde(rename = "cryptoHashConfig", default)]
        pub crypto_hash_config: Option<crate::schemas::CryptoHashConfig>,
        #[doc = "Config for date shift."]
        #[serde(rename = "dateShiftConfig", default)]
        pub date_shift_config: Option<crate::schemas::DateShiftConfig>,
        #[doc = "InfoTypes to apply this transformation to. If this is not specified, the\ntransformation applies to any info_type."]
        #[serde(rename = "infoTypes", default)]
        pub info_types: Option<Vec<String>>,
        #[doc = "Config for text redaction."]
        #[serde(rename = "redactConfig", default)]
        pub redact_config: Option<crate::schemas::RedactConfig>,
        #[doc = "Config for replace with InfoType."]
        #[serde(rename = "replaceWithInfoTypeConfig", default)]
        pub replace_with_info_type_config: Option<crate::schemas::ReplaceWithInfoTypeConfig>,
    }
    impl ::field_selector::FieldSelector for InfoTypeTransformation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IngestMessageRequest {
        #[doc = "HL7v2 message to ingest."]
        #[serde(rename = "message", default)]
        pub message: Option<crate::schemas::Message>,
    }
    impl ::field_selector::FieldSelector for IngestMessageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IngestMessageResponse {
        #[doc = "HL7v2 ACK message."]
        #[serde(rename = "hl7Ack", default)]
        pub hl_7_ack: Option<Vec<u8>>,
        #[doc = "Created message resource."]
        #[serde(rename = "message", default)]
        pub message: Option<crate::schemas::Message>,
    }
    impl ::field_selector::FieldSelector for IngestMessageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListDatasetsResponse {
        #[doc = "The first page of datasets."]
        #[serde(rename = "datasets", default)]
        pub datasets: Option<Vec<crate::schemas::Dataset>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListDatasetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListDicomStoresResponse {
        #[doc = "The returned DICOM stores. Won't be more DICOM stores than the value of\npage_size in the request."]
        #[serde(rename = "dicomStores", default)]
        pub dicom_stores: Option<Vec<crate::schemas::DicomStore>>,
        #[doc = "Token to retrieve the next page of results or empty if there are no more\nresults in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListDicomStoresResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListFhirStoresResponse {
        #[doc = "The returned FHIR stores. Won't be more FHIR stores than the value of\npage_size in the request."]
        #[serde(rename = "fhirStores", default)]
        pub fhir_stores: Option<Vec<crate::schemas::FhirStore>>,
        #[doc = "Token to retrieve the next page of results or empty if there are no more\nresults in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListFhirStoresResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListHl7V2StoresResponse {
        #[doc = "The returned HL7v2 stores. Won't be more HL7v2 stores than the value of\npage_size in the request."]
        #[serde(rename = "hl7V2Stores", default)]
        pub hl_7v2_stores: Option<Vec<crate::schemas::Hl7V2Store>>,
        #[doc = "Token to retrieve the next page of results or empty if there are no more\nresults in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListHl7V2StoresResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListLocationsResponse {
        #[doc = "A list of locations that matches the specified filter in the request."]
        #[serde(rename = "locations", default)]
        pub locations: Option<Vec<crate::schemas::Location>>,
        #[doc = "The standard List next-page token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListLocationsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListMessagesResponse {
        #[doc = "The returned message names. Won't be more values than the value of\npage_size in the request."]
        #[serde(rename = "messages", default)]
        pub messages: Option<Vec<String>>,
        #[doc = "Token to retrieve the next page of results or empty if there are no more\nresults in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListMessagesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(rename = "operations", default)]
        pub operations: Option<Vec<crate::schemas::Operation>>,
    }
    impl ::field_selector::FieldSelector for ListOperationsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Location {
        #[doc = "The friendly name for this location, typically a nearby city name.\nFor example, \"Tokyo\"."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "Cross-service attributes for the location. For example\n\n    {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
        #[serde(rename = "locationId", default)]
        pub location_id: Option<String>,
        #[doc = "Service-specific metadata. For example the available capacity at the given\nlocation."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Resource name for the location, which may vary between implementations.\nFor example: `\"projects/example-project/locations/us-east1\"`"]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Location {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Message {
        #[doc = "Output only. The datetime when the message was created. Set by the server."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Raw message bytes."]
        #[serde(rename = "data", default)]
        pub data: Option<Vec<u8>>,
        #[doc = "User-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding\nof maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression:\n\\p{Ll}\\p{Lo}{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have\na UTF-8 encoding of maximum 128 bytes, and must conform to the\nfollowing PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The message type and trigger event for this message. MSH-9."]
        #[serde(rename = "messageType", default)]
        pub message_type: Option<String>,
        #[doc = "Resource name of the Message, of the form\n`projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`.\nAssigned by the server."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Output only. The parsed version of the raw message data."]
        #[serde(rename = "parsedData", default)]
        pub parsed_data: Option<crate::schemas::ParsedData>,
        #[doc = "All patient IDs listed in the PID-2, PID-3, and PID-4 segments of this\nmessage."]
        #[serde(rename = "patientIds", default)]
        pub patient_ids: Option<Vec<crate::schemas::PatientId>>,
        #[doc = "The hospital that this message came from. MSH-4."]
        #[serde(rename = "sendFacility", default)]
        pub send_facility: Option<String>,
        #[doc = "The datetime the sending application sent this message. MSH-7."]
        #[serde(rename = "sendTime", default)]
        pub send_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for Message {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NotificationConfig {
        #[doc = "The [Cloud Pub/Sub](https://cloud.google.com/pubsub/docs/) topic that\nnotifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name.\nPubsubMessage.MessageId is the ID of this message. It is guaranteed to be\nunique within the topic.\nPubsubMessage.PublishTime is the time at which the message was published.\nNotifications are only sent if the topic is\nnon-empty. [Topic\nnames](https://cloud.google.com/pubsub/docs/overview#names) must be scoped\nto a project. cloud-healthcare@system.gserviceaccount.com must have\npublisher permissions on the given Cloud Pub/Sub topic. Not having adequate\npermissions will cause the calls that send notifications to fail."]
        #[serde(rename = "pubsubTopic", default)]
        pub pubsub_topic: Option<String>,
    }
    impl ::field_selector::FieldSelector for NotificationConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Operation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OperationMetadata {
        #[doc = "The name of the API method that initiated the operation."]
        #[serde(rename = "apiMethodName", default)]
        pub api_method_name: Option<String>,
        #[serde(rename = "counter", default)]
        pub counter: Option<crate::schemas::ProgressCounter>,
        #[doc = "The time at which the operation was created by the API."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "The time at which execution was completed."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ParsedData {
        #[serde(rename = "segments", default)]
        pub segments: Option<Vec<crate::schemas::Segment>>,
    }
    impl ::field_selector::FieldSelector for ParsedData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ParserConfig {
        #[doc = "Determines whether messages with no header are allowed."]
        #[serde(rename = "allowNullHeader", default)]
        pub allow_null_header: Option<bool>,
        #[doc = "Byte(s) to be used as the segment terminator. If this is unset, '\\r' will\nbe used as segment terminator."]
        #[serde(rename = "segmentTerminator", default)]
        pub segment_terminator: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for ParserConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PatientId {
        #[doc = "ID type, e.g. MRN or NHS."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The patient's unique identifier."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for PatientId {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(rename = "auditConfigs", default)]
        pub audit_configs: Option<Vec<crate::schemas::AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`.\n`bindings` with no members will result in an error."]
        #[serde(rename = "bindings", default)]
        pub bindings: Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a policy from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform policy updates in order to avoid race\nconditions: An `etag` is returned in the response to `getIamPolicy`, and\nsystems are expected to put that etag in the request to `setIamPolicy` to\nensure that their change will be applied to the same version of the policy.\n\nIf no `etag` is provided in the call to `setIamPolicy`, then the existing\npolicy is overwritten."]
        #[serde(rename = "etag", default)]
        pub etag: Option<Vec<u8>>,
        #[doc = "Deprecated."]
        #[serde(rename = "version", default)]
        pub version: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Policy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ProgressCounter {
        #[doc = "The number of units that failed in the operation."]
        #[serde(rename = "failure", default)]
        #[serde(with = "crate::parsed_string")]
        pub failure: Option<i64>,
        #[doc = "The number of units that are pending in the operation."]
        #[serde(rename = "pending", default)]
        #[serde(with = "crate::parsed_string")]
        pub pending: Option<i64>,
        #[doc = "The number of units that succeeded in the operation."]
        #[serde(rename = "success", default)]
        #[serde(with = "crate::parsed_string")]
        pub success: Option<i64>,
    }
    impl ::field_selector::FieldSelector for ProgressCounter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RedactConfig;
    impl ::field_selector::FieldSelector for RedactConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceWithInfoTypeConfig;
    impl ::field_selector::FieldSelector for ReplaceWithInfoTypeConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SchemaConfigSchemaType {
        #[doc = "No schema type specified. Same as `LOSSLESS`."]
        SchemaTypeUnspecified,
        #[doc = "A data-driven schema generated from the fields present in the FHIR data\nbeing exported, with no additional simplification."]
        Lossless,
        #[doc = "Analytics schema defined by the FHIR community.\nSee https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md."]
        Analytics,
    }
    impl SchemaConfigSchemaType {
        pub fn as_str(self) -> &'static str {
            match self {
                SchemaConfigSchemaType::SchemaTypeUnspecified => "SCHEMA_TYPE_UNSPECIFIED",
                SchemaConfigSchemaType::Lossless => "LOSSLESS",
                SchemaConfigSchemaType::Analytics => "ANALYTICS",
            }
        }
    }
    impl ::std::fmt::Display for SchemaConfigSchemaType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SchemaConfigSchemaType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SchemaConfigSchemaType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCHEMA_TYPE_UNSPECIFIED" => SchemaConfigSchemaType::SchemaTypeUnspecified,
                "LOSSLESS" => SchemaConfigSchemaType::Lossless,
                "ANALYTICS" => SchemaConfigSchemaType::Analytics,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SchemaConfig {
        #[doc = "The depth for all recursive structures in the output analytics\nschema. For example, `concept` in the CodeSystem resource is a recursive\nstructure; when the depth is 2, the CodeSystem table will have a column\ncalled `concept.concept` but not `concept.concept.concept`. If not\nspecified or set to 0, the server will use the default value 2."]
        #[serde(rename = "recursiveStructureDepth", default)]
        #[serde(with = "crate::parsed_string")]
        pub recursive_structure_depth: Option<i64>,
        #[doc = "Specifies the output schema type. If unspecified, the default is\n`LOSSLESS`."]
        #[serde(rename = "schemaType", default)]
        pub schema_type: Option<crate::schemas::SchemaConfigSchemaType>,
    }
    impl ::field_selector::FieldSelector for SchemaConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SearchResourcesRequest {
        #[doc = "The FHIR resource type to search, such as Patient or Observation. For a\ncomplete list, see the [FHIR Resource\nIndex](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html)."]
        #[serde(rename = "resourceType", default)]
        pub resource_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchResourcesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Segment {
        #[doc = "A mapping from the positional location to the value.\nThe key string uses zero-based indexes separated by dots to identify\nFields, components and sub-components. A bracket notation is also used to\nidentify different instances of a repeated field.\nRegex for key: (\\d+)(\\[\\d+\\])?(.\\d+)?(.\\d+)?\n\nExamples of (key, value) pairs:\n- (0.1, \"foo\"): Component 1 of Field 0 has the value \"foo\".\n- (1.1.2, \"bar\"): Sub-component 2 of Component 1 of field 1 has the value\n\"bar\".\n- (1[2].1, \"baz\"): Component 1 of Instance 2 of Field 1, which is repeated,\nhas the value \"baz\"."]
        #[serde(rename = "fields", default)]
        pub fields: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A string that indicates the type of segment, e.g., EVN, PID."]
        #[serde(rename = "segmentId", default)]
        pub segment_id: Option<String>,
        #[doc = "Set ID for segments that can be in a set. This can be empty if it is\nmissing or it is not applicable."]
        #[serde(rename = "setId", default)]
        pub set_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Segment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of\nthe policy is limited to a few 10s of KB. An empty policy is a\nvalid policy but certain Cloud Platform services (such as Projects)\nmight reject them."]
        #[serde(rename = "policy", default)]
        pub policy: Option<crate::schemas::Policy>,
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only\nthe fields in the mask will be modified. If no mask is provided, the\nfollowing default mask is used:\npaths: \"bindings, etag\"\nThis field is only used by Cloud IAM."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: Option<String>,
    }
    impl ::field_selector::FieldSelector for SetIamPolicyRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TagFilterList {
        #[doc = "Tags to be filtered. Tags must be DICOM Data Elements, File Meta\nElements, or Directory Structuring Elements, as defined at:\nhttp://dicom.nema.org/medical/dicom/current/output/html/part06.html#table_6-1,.\nThey may be provided by \"Keyword\" or \"Tag\". For example \"PatientID\",\n\"00100010\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TagFilterList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with\nwildcards (such as '*' or 'storage.*') are not allowed. For more\ninformation see\n[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TestIamPermissionsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is\nallowed."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TestIamPermissionsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextConfig {
        #[doc = "The transformations to apply to the detected data."]
        #[serde(rename = "transformations", default)]
        pub transformations: Option<Vec<crate::schemas::InfoTypeTransformation>>,
    }
    impl ::field_selector::FieldSelector for TextConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::projects::ProjectsActions<A> {
        crate::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod projects {
    pub mod params {}
    pub struct ProjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
        #[doc = "Actions that can be performed on the locations resource"]
        pub fn locations(&self) -> locations::LocationsActions<A> {
            locations::LocationsActions
        }
    }
    pub mod locations {
        pub mod params {}
        pub struct LocationsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> LocationsActions<'a, A> {
            #[doc = "Gets information about a location."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                }
            }
            #[doc = "Lists information about the supported locations for this service."]
            pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                    filter: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Actions that can be performed on the datasets resource"]
            pub fn datasets(&self) -> datasets::DatasetsActions<A> {
                datasets::DatasetsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Location, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://healthcare.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
            filter: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "The standard list filter."]
            pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The standard list page size."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The standard list page token."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn iter_locations<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "locations")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListLocationsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://healthcare.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                output.push_str(&self.name);
                output.push_str("/locations");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        pub mod datasets {
            pub mod params {}
            pub struct DatasetsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> DatasetsActions<'a, A> {
                #[doc = "Creates a new health dataset. Results are returned through the\nOperation interface which returns either an\n`Operation.response` which contains a Dataset or\n`Operation.error`. The metadata\nfield type is OperationMetadata.\nA Google Cloud Platform project can contain up to 500 datasets across all\nregions."]
                pub fn create(
                    &self,
                    request: crate::schemas::Dataset,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                        dataset_id: None,
                    }
                }
                #[doc = "Creates a new dataset containing de-identified data from the source\ndataset. The metadata field type\nis OperationMetadata.\nIf the request is successful, the\nresponse field type is\nDeidentifySummary.\nIf errors occur,\ndetails field type is\nDeidentifyErrorDetails."]
                pub fn deidentify(
                    &self,
                    request: crate::schemas::DeidentifyDatasetRequest,
                    source_dataset: impl Into<String>,
                ) -> DeidentifyRequestBuilder<A> {
                    DeidentifyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        source_dataset: source_dataset.into(),
                    }
                }
                #[doc = "Deletes the specified health dataset and all data contained in the dataset.\nDeleting a dataset does not affect the sources from which the dataset was\nimported (if any)."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Gets any metadata associated with a dataset."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                pub fn get_iam_policy(
                    &self,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder<A> {
                    GetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                        options_requested_policy_version: None,
                    }
                }
                #[doc = "Lists the health datasets in the current project."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates dataset metadata."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Dataset,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder<A> {
                    PatchRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder<A> {
                    SetIamPolicyRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder<A> {
                    TestIamPermissionsRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        resource: resource.into(),
                    }
                }
                #[doc = "Actions that can be performed on the dicom_stores resource"]
                pub fn dicom_stores(&self) -> dicom_stores::DicomStoresActions<A> {
                    dicom_stores::DicomStoresActions
                }
                #[doc = "Actions that can be performed on the fhir_stores resource"]
                pub fn fhir_stores(&self) -> fhir_stores::FhirStoresActions<A> {
                    fhir_stores::FhirStoresActions
                }
                #[doc = "Actions that can be performed on the hl_7v2_stores resource"]
                pub fn hl_7v2_stores(&self) -> hl_7v2_stores::Hl7V2StoresActions<A> {
                    hl_7v2_stores::Hl7V2StoresActions
                }
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(&self) -> operations::OperationsActions<A> {
                    operations::OperationsActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Dataset,
                parent: String,
                dataset_id: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "The ID of the dataset that is being created.\nThe string must match the following regex: `[\\p{L}\\p{N}_\\-\\.]{1,256}`."]
                pub fn dataset_id(&mut self, value: impl Into<String>) -> &mut Self {
                    self.dataset_id = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.parent);
                    output.push_str("/datasets");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("datasetId", &self.dataset_id)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeidentifyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::DeidentifyDatasetRequest,
                source_dataset: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> DeidentifyRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.source_dataset);
                    output.push_str(":deidentify");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Dataset, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                resource: String,
                options_requested_policy_version: Option<i32>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetIamPolicyRequestBuilder<'a, A> {
                #[doc = "Optional. The policy format version to be returned.\nAcceptable values are 0 and 1.\nIf the value is 0, or the field is omitted, policy format version 1 will be\nreturned."]
                pub fn options_requested_policy_version(&mut self, value: i32) -> &mut Self {
                    self.options_requested_policy_version = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.resource);
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "options.requestedPolicyVersion",
                        &self.options_requested_policy_version,
                    )]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "The maximum number of items to return. Capped to 100 if not specified.\nMay not be larger than 1000."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The next_page_token value returned from a previous List request, if any."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn iter_datasets<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "datasets")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListDatasetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.parent);
                    output.push_str("/datasets");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Dataset,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                #[doc = "The update mask applies to the resource. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
                pub fn update_mask(&mut self, value: impl Into<String>) -> &mut Self {
                    self.update_mask = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Dataset, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    let req = req.query(&[("updateMask", &self.update_mask)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> SetIamPolicyRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>> {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.resource);
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> TestIamPermissionsRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                    self.xgafv = Some(value);
                    self
                }
                pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://healthcare.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    output.push_str(&self.resource);
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            pub mod dicom_stores {
                pub mod params {}
                pub struct DicomStoresActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> DicomStoresActions<'a, A> {
                    #[doc = "Creates a new DICOM store within the parent dataset."]
                    pub fn create(
                        &self,
                        request: crate::schemas::DicomStore,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            dicom_store_id: None,
                        }
                    }
                    #[doc = "Deletes the specified DICOM store and removes all images that are contained\nwithin it."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Exports data to the specified destination by copying it from the DICOM\nstore.\nThe metadata field type is\nOperationMetadata."]
                    pub fn export(
                        &self,
                        request: crate::schemas::ExportDicomDataRequest,
                        name: impl Into<String>,
                    ) -> ExportRequestBuilder<A> {
                        ExportRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the specified DICOM store."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder<A> {
                        GetIamPolicyRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Imports data into the DICOM store by copying it from the specified source.\nFor errors, the Operation will be populated with error details (in the form\nof ImportDicomDataErrorDetails in error.details), which will hold\nfiner-grained error information. Errors are also logged to Stackdriver\n(see [Viewing logs](/healthcare/docs/how-tos/stackdriver-logging)).\nThe metadata field type is\nOperationMetadata."]
                    pub fn import(
                        &self,
                        request: crate::schemas::ImportDicomDataRequest,
                        name: impl Into<String>,
                    ) -> ImportRequestBuilder<A> {
                        ImportRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists the DICOM stores in the given dataset."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the specified DICOM store."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::DicomStore,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder<A> {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                    #[doc = "SearchForInstances returns a list of matching instances. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6."]
                    pub fn search_for_instances(
                        &self,
                        parent: impl Into<String>,
                        dicom_web_path: impl Into<String>,
                    ) -> SearchForInstancesRequestBuilder<A> {
                        SearchForInstancesRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            dicom_web_path: dicom_web_path.into(),
                        }
                    }
                    #[doc = "SearchForSeries returns a list of matching series. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6."]
                    pub fn search_for_series(
                        &self,
                        parent: impl Into<String>,
                        dicom_web_path: impl Into<String>,
                    ) -> SearchForSeriesRequestBuilder<A> {
                        SearchForSeriesRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            dicom_web_path: dicom_web_path.into(),
                        }
                    }
                    #[doc = "SearchForStudies returns a list of matching studies. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6."]
                    pub fn search_for_studies(
                        &self,
                        parent: impl Into<String>,
                        dicom_web_path: impl Into<String>,
                    ) -> SearchForStudiesRequestBuilder<A> {
                        SearchForStudiesRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            dicom_web_path: dicom_web_path.into(),
                        }
                    }
                    #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy."]
                    pub fn set_iam_policy(
                        &self,
                        request: crate::schemas::SetIamPolicyRequest,
                        resource: impl Into<String>,
                    ) -> SetIamPolicyRequestBuilder<A> {
                        SetIamPolicyRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                        }
                    }
                    #[doc = "StoreInstances stores DICOM instances associated with study instance unique\nidentifiers (SUID). See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5."]
                    pub fn store_instances(
                        &self,
                        request: crate::schemas::HttpBody,
                        parent: impl Into<String>,
                        dicom_web_path: impl Into<String>,
                    ) -> StoreInstancesRequestBuilder<A> {
                        StoreInstancesRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            dicom_web_path: dicom_web_path.into(),
                        }
                    }
                    #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                    pub fn test_iam_permissions(
                        &self,
                        request: crate::schemas::TestIamPermissionsRequest,
                        resource: impl Into<String>,
                    ) -> TestIamPermissionsRequestBuilder<A> {
                        TestIamPermissionsRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the studies resource"]
                    pub fn studies(&self) -> studies::StudiesActions<A> {
                        studies::StudiesActions
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::DicomStore,
                    parent: String,
                    dicom_store_id: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "The ID of the DICOM store that is being created.\nAny string value up to 256 characters in length."]
                    pub fn dicom_store_id(&mut self, value: impl Into<String>) -> &mut Self {
                        self.dicom_store_id = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::DicomStore, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/dicomStores");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("dicomStoreId", &self.dicom_store_id)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ExportRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::ExportDicomDataRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ExportRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output.push_str(":export");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::DicomStore, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    resource: String,
                    options_requested_policy_version: Option<i32>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetIamPolicyRequestBuilder<'a, A> {
                    #[doc = "Optional. The policy format version to be returned.\nAcceptable values are 0 and 1.\nIf the value is 0, or the field is omitted, policy format version 1 will be\nreturned."]
                    pub fn options_requested_policy_version(&mut self, value: i32) -> &mut Self {
                        self.options_requested_policy_version = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":getIamPolicy");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ImportRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::ImportDicomDataRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ImportRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output.push_str(":import");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Restricts stores returned to those matching a filter. Syntax:\nhttps://cloud.google.com/appengine/docs/standard/python/search/query_strings\nOnly filtering on labels is supported, for example `labels.key=value`."]
                    pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Limit on the number of DICOM stores to return in a single response.\nIf zero the default page size of 100 is used."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from the previous List request, if any."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_dicom_stores<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "dicomStores")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListDicomStoresResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/dicomStores");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::DicomStore,
                    name: String,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                    #[doc = "The update mask applies to the resource. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
                    pub fn update_mask(&mut self, value: impl Into<String>) -> &mut Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::DicomStore, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SearchForInstancesRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    dicom_web_path: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SearchForInstancesRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/dicomWeb/");
                        output.push_str(&self.dicom_web_path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SearchForSeriesRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    dicom_web_path: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SearchForSeriesRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/dicomWeb/");
                        output.push_str(&self.dicom_web_path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SearchForStudiesRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    dicom_web_path: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SearchForStudiesRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/dicomWeb/");
                        output.push_str(&self.dicom_web_path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SetIamPolicyRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SetIamPolicyRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":setIamPolicy");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct StoreInstancesRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::HttpBody,
                    parent: String,
                    dicom_web_path: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> StoreInstancesRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/dicomWeb/");
                        output.push_str(&self.dicom_web_path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct TestIamPermissionsRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> TestIamPermissionsRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::TestIamPermissionsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":testIamPermissions");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                pub mod studies {
                    pub mod params {}
                    pub struct StudiesActions<'a, A> {
                        pub(super) reqwest: &'a reqwest::Client,
                        pub(super) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> StudiesActions<'a, A> {
                        #[doc = "DeleteStudy deletes all instances within the given study. Delete requests\nare equivalent to the GET requests specified in the WADO-RS standard."]
                        pub fn delete(
                            &self,
                            parent: impl Into<String>,
                            dicom_web_path: impl Into<String>,
                        ) -> DeleteRequestBuilder<A> {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                dicom_web_path: dicom_web_path.into(),
                            }
                        }
                        #[doc = "RetrieveStudyMetadata returns instance associated with the given study\npresented as metadata with the bulk data removed. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                        pub fn retrieve_metadata(
                            &self,
                            parent: impl Into<String>,
                            dicom_web_path: impl Into<String>,
                        ) -> RetrieveMetadataRequestBuilder<A> {
                            RetrieveMetadataRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                dicom_web_path: dicom_web_path.into(),
                            }
                        }
                        #[doc = "RetrieveStudy returns all instances within the given study. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                        pub fn retrieve_study(
                            &self,
                            parent: impl Into<String>,
                            dicom_web_path: impl Into<String>,
                        ) -> RetrieveStudyRequestBuilder<A> {
                            RetrieveStudyRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                dicom_web_path: dicom_web_path.into(),
                            }
                        }
                        #[doc = "SearchForInstances returns a list of matching instances. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6."]
                        pub fn search_for_instances(
                            &self,
                            parent: impl Into<String>,
                            dicom_web_path: impl Into<String>,
                        ) -> SearchForInstancesRequestBuilder<A> {
                            SearchForInstancesRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                dicom_web_path: dicom_web_path.into(),
                            }
                        }
                        #[doc = "SearchForSeries returns a list of matching series. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6."]
                        pub fn search_for_series(
                            &self,
                            parent: impl Into<String>,
                            dicom_web_path: impl Into<String>,
                        ) -> SearchForSeriesRequestBuilder<A> {
                            SearchForSeriesRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                dicom_web_path: dicom_web_path.into(),
                            }
                        }
                        #[doc = "StoreInstances stores DICOM instances associated with study instance unique\nidentifiers (SUID). See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.5."]
                        pub fn store_instances(
                            &self,
                            request: crate::schemas::HttpBody,
                            parent: impl Into<String>,
                            dicom_web_path: impl Into<String>,
                        ) -> StoreInstancesRequestBuilder<A> {
                            StoreInstancesRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                dicom_web_path: dicom_web_path.into(),
                            }
                        }
                        #[doc = "Actions that can be performed on the series resource"]
                        pub fn series(&self) -> series::SeriesActions<A> {
                            series::SeriesActions
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        dicom_web_path: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/dicomWeb/");
                            output.push_str(&self.dicom_web_path);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct RetrieveMetadataRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        dicom_web_path: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> RetrieveMetadataRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/dicomWeb/");
                            output.push_str(&self.dicom_web_path);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct RetrieveStudyRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        dicom_web_path: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> RetrieveStudyRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/dicomWeb/");
                            output.push_str(&self.dicom_web_path);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct SearchForInstancesRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        dicom_web_path: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> SearchForInstancesRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/dicomWeb/");
                            output.push_str(&self.dicom_web_path);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct SearchForSeriesRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        dicom_web_path: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> SearchForSeriesRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/dicomWeb/");
                            output.push_str(&self.dicom_web_path);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct StoreInstancesRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        parent: String,
                        dicom_web_path: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> StoreInstancesRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/dicomWeb/");
                            output.push_str(&self.dicom_web_path);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    pub mod series {
                        pub mod params {}
                        pub struct SeriesActions<'a, A> {
                            pub(super) reqwest: &'a reqwest::Client,
                            pub(super) auth: &'a std::sync::Mutex<A>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> SeriesActions<'a, A> {
                            #[doc = "DeleteSeries deletes all instances within the given study and series.\nDelete requests are equivalent to the GET requests specified in the WADO-RS\nstandard."]
                            pub fn delete(
                                &self,
                                parent: impl Into<String>,
                                dicom_web_path: impl Into<String>,
                            ) -> DeleteRequestBuilder<A> {
                                DeleteRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    dicom_web_path: dicom_web_path.into(),
                                }
                            }
                            #[doc = "RetrieveSeriesMetadata returns instance associated with the given study and\nseries, presented as metadata with the bulk data removed. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                            pub fn retrieve_metadata(
                                &self,
                                parent: impl Into<String>,
                                dicom_web_path: impl Into<String>,
                            ) -> RetrieveMetadataRequestBuilder<A> {
                                RetrieveMetadataRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    dicom_web_path: dicom_web_path.into(),
                                }
                            }
                            #[doc = "RetrieveSeries returns all instances within the given study and series. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                            pub fn retrieve_series(
                                &self,
                                parent: impl Into<String>,
                                dicom_web_path: impl Into<String>,
                            ) -> RetrieveSeriesRequestBuilder<A> {
                                RetrieveSeriesRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    dicom_web_path: dicom_web_path.into(),
                                }
                            }
                            #[doc = "SearchForInstances returns a list of matching instances. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.6."]
                            pub fn search_for_instances(
                                &self,
                                parent: impl Into<String>,
                                dicom_web_path: impl Into<String>,
                            ) -> SearchForInstancesRequestBuilder<A> {
                                SearchForInstancesRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    dicom_web_path: dicom_web_path.into(),
                                }
                            }
                            #[doc = "Actions that can be performed on the instances resource"]
                            pub fn instances(&self) -> instances::InstancesActions<A> {
                                instances::InstancesActions
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct DeleteRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            parent: String,
                            dicom_web_path: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "Data format for response."]
                            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                self.alt = Some(value);
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "Selector specifying which fields to include in a partial response."]
                            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                self.fields = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(
                                &mut self,
                                value: impl Into<String>,
                            ) -> &mut Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                self.xgafv = Some(value);
                                self
                            }
                            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                self._execute()
                            }
                            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                            pub fn execute_text(
                                self,
                            ) -> Result<String, Box<dyn ::std::error::Error>>
                            {
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.text()?)
                            }
                            pub fn execute_debug(
                                self,
                            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                            {
                                self.execute()
                            }
                            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                if self.fields.is_none() {
                                    self.fields = Some(T::field_selector());
                                }
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output = "https://healthcare.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                output.push_str(&self.parent);
                                output.push_str("/dicomWeb/");
                                output.push_str(&self.dicom_web_path);
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                                let req = req.query(&[("access_token", &self.access_token)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("callback", &self.callback)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                let req = req.query(&[("uploadType", &self.upload_type)]);
                                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let mut auth = self.auth.lock().unwrap();
                                let req = req.bearer_auth(
                                    auth.token::<_, &str>(&[
                                        "https://www.googleapis.com/auth/cloud-platform",
                                    ])
                                    .unwrap()
                                    .access_token,
                                );
                                req
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct RetrieveMetadataRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            parent: String,
                            dicom_web_path: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> RetrieveMetadataRequestBuilder<'a, A> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "Data format for response."]
                            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                self.alt = Some(value);
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "Selector specifying which fields to include in a partial response."]
                            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                self.fields = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(
                                &mut self,
                                value: impl Into<String>,
                            ) -> &mut Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                self.xgafv = Some(value);
                                self
                            }
                            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                self._execute()
                            }
                            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                            pub fn execute_text(
                                self,
                            ) -> Result<String, Box<dyn ::std::error::Error>>
                            {
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.text()?)
                            }
                            pub fn execute_debug(
                                self,
                            ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                            {
                                self.execute()
                            }
                            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                if self.fields.is_none() {
                                    self.fields = Some(T::field_selector());
                                }
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output = "https://healthcare.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                output.push_str(&self.parent);
                                output.push_str("/dicomWeb/");
                                output.push_str(&self.dicom_web_path);
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("access_token", &self.access_token)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("callback", &self.callback)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                let req = req.query(&[("uploadType", &self.upload_type)]);
                                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let mut auth = self.auth.lock().unwrap();
                                let req = req.bearer_auth(
                                    auth.token::<_, &str>(&[
                                        "https://www.googleapis.com/auth/cloud-platform",
                                    ])
                                    .unwrap()
                                    .access_token,
                                );
                                req
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct RetrieveSeriesRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            parent: String,
                            dicom_web_path: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> RetrieveSeriesRequestBuilder<'a, A> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "Data format for response."]
                            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                self.alt = Some(value);
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "Selector specifying which fields to include in a partial response."]
                            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                self.fields = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(
                                &mut self,
                                value: impl Into<String>,
                            ) -> &mut Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                self.xgafv = Some(value);
                                self
                            }
                            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                self._execute()
                            }
                            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                            pub fn execute_text(
                                self,
                            ) -> Result<String, Box<dyn ::std::error::Error>>
                            {
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.text()?)
                            }
                            pub fn execute_debug(
                                self,
                            ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                            {
                                self.execute()
                            }
                            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                if self.fields.is_none() {
                                    self.fields = Some(T::field_selector());
                                }
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output = "https://healthcare.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                output.push_str(&self.parent);
                                output.push_str("/dicomWeb/");
                                output.push_str(&self.dicom_web_path);
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("access_token", &self.access_token)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("callback", &self.callback)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                let req = req.query(&[("uploadType", &self.upload_type)]);
                                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let mut auth = self.auth.lock().unwrap();
                                let req = req.bearer_auth(
                                    auth.token::<_, &str>(&[
                                        "https://www.googleapis.com/auth/cloud-platform",
                                    ])
                                    .unwrap()
                                    .access_token,
                                );
                                req
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct SearchForInstancesRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            parent: String,
                            dicom_web_path: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> SearchForInstancesRequestBuilder<'a, A> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "Data format for response."]
                            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                self.alt = Some(value);
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "Selector specifying which fields to include in a partial response."]
                            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                self.fields = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(
                                &mut self,
                                value: impl Into<String>,
                            ) -> &mut Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                self.xgafv = Some(value);
                                self
                            }
                            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                self._execute()
                            }
                            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                            pub fn execute_text(
                                self,
                            ) -> Result<String, Box<dyn ::std::error::Error>>
                            {
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.text()?)
                            }
                            pub fn execute_debug(
                                self,
                            ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                            {
                                self.execute()
                            }
                            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                if self.fields.is_none() {
                                    self.fields = Some(T::field_selector());
                                }
                                let req = self._request(&self._path());
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output = "https://healthcare.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                output.push_str(&self.parent);
                                output.push_str("/dicomWeb/");
                                output.push_str(&self.dicom_web_path);
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("access_token", &self.access_token)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("callback", &self.callback)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                let req = req.query(&[("uploadType", &self.upload_type)]);
                                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let mut auth = self.auth.lock().unwrap();
                                let req = req.bearer_auth(
                                    auth.token::<_, &str>(&[
                                        "https://www.googleapis.com/auth/cloud-platform",
                                    ])
                                    .unwrap()
                                    .access_token,
                                );
                                req
                            }
                        }
                        pub mod instances {
                            pub mod params {}
                            pub struct InstancesActions<'a, A> {
                                pub(super) reqwest: &'a reqwest::Client,
                                pub(super) auth: &'a std::sync::Mutex<A>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> InstancesActions<'a, A> {
                                #[doc = "DeleteInstance deletes an instance associated with the given study, series,\nand SOP Instance UID. Delete requests are equivalent to the GET requests\nspecified in the WADO-RS standard."]
                                pub fn delete(
                                    &self,
                                    parent: impl Into<String>,
                                    dicom_web_path: impl Into<String>,
                                ) -> DeleteRequestBuilder<A> {
                                    DeleteRequestBuilder {
                                        reqwest: &self.reqwest,
                                        auth: &self.auth,
                                        access_token: None,
                                        alt: None,
                                        callback: None,
                                        fields: None,
                                        key: None,
                                        oauth_token: None,
                                        pretty_print: None,
                                        quota_user: None,
                                        upload_protocol: None,
                                        upload_type: None,
                                        xgafv: None,
                                        parent: parent.into(),
                                        dicom_web_path: dicom_web_path.into(),
                                    }
                                }
                                #[doc = "RetrieveInstance returns instance associated with the given study, series,\nand SOP Instance UID. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                                pub fn retrieve_instance(
                                    &self,
                                    parent: impl Into<String>,
                                    dicom_web_path: impl Into<String>,
                                ) -> RetrieveInstanceRequestBuilder<A>
                                {
                                    RetrieveInstanceRequestBuilder {
                                        reqwest: &self.reqwest,
                                        auth: &self.auth,
                                        access_token: None,
                                        alt: None,
                                        callback: None,
                                        fields: None,
                                        key: None,
                                        oauth_token: None,
                                        pretty_print: None,
                                        quota_user: None,
                                        upload_protocol: None,
                                        upload_type: None,
                                        xgafv: None,
                                        parent: parent.into(),
                                        dicom_web_path: dicom_web_path.into(),
                                    }
                                }
                                #[doc = "RetrieveInstanceMetadata returns instance associated with the given study,\nseries, and SOP Instance UID presented as metadata with the bulk data\nremoved. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                                pub fn retrieve_metadata(
                                    &self,
                                    parent: impl Into<String>,
                                    dicom_web_path: impl Into<String>,
                                ) -> RetrieveMetadataRequestBuilder<A>
                                {
                                    RetrieveMetadataRequestBuilder {
                                        reqwest: &self.reqwest,
                                        auth: &self.auth,
                                        access_token: None,
                                        alt: None,
                                        callback: None,
                                        fields: None,
                                        key: None,
                                        oauth_token: None,
                                        pretty_print: None,
                                        quota_user: None,
                                        upload_protocol: None,
                                        upload_type: None,
                                        xgafv: None,
                                        parent: parent.into(),
                                        dicom_web_path: dicom_web_path.into(),
                                    }
                                }
                                #[doc = "RetrieveRenderedInstance returns instance associated with the given study,\nseries, and SOP Instance UID in an acceptable Rendered Media Type. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                                pub fn retrieve_rendered(
                                    &self,
                                    parent: impl Into<String>,
                                    dicom_web_path: impl Into<String>,
                                ) -> RetrieveRenderedRequestBuilder<A>
                                {
                                    RetrieveRenderedRequestBuilder {
                                        reqwest: &self.reqwest,
                                        auth: &self.auth,
                                        access_token: None,
                                        alt: None,
                                        callback: None,
                                        fields: None,
                                        key: None,
                                        oauth_token: None,
                                        pretty_print: None,
                                        quota_user: None,
                                        upload_protocol: None,
                                        upload_type: None,
                                        xgafv: None,
                                        parent: parent.into(),
                                        dicom_web_path: dicom_web_path.into(),
                                    }
                                }
                                #[doc = "Actions that can be performed on the frames resource"]
                                pub fn frames(&self) -> frames::FramesActions<A> {
                                    frames::FramesActions
                                }
                            }
                            #[derive(Debug, Clone)]
                            pub struct DeleteRequestBuilder<'a, A> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                parent: String,
                                dicom_web_path: String,
                                access_token: Option<String>,
                                alt: Option<crate::params::Alt>,
                                callback: Option<String>,
                                fields: Option<String>,
                                key: Option<String>,
                                oauth_token: Option<String>,
                                pretty_print: Option<bool>,
                                quota_user: Option<String>,
                                upload_protocol: Option<String>,
                                upload_type: Option<String>,
                                xgafv: Option<crate::params::Xgafv>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                                #[doc = "OAuth access token."]
                                pub fn access_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.access_token = Some(value.into());
                                    self
                                }
                                #[doc = "Data format for response."]
                                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                    self.alt = Some(value);
                                    self
                                }
                                #[doc = "JSONP"]
                                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.callback = Some(value.into());
                                    self
                                }
                                #[doc = "Selector specifying which fields to include in a partial response."]
                                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.fields = Some(value.into());
                                    self
                                }
                                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.key = Some(value.into());
                                    self
                                }
                                #[doc = "OAuth 2.0 token for the current user."]
                                pub fn oauth_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.oauth_token = Some(value.into());
                                    self
                                }
                                #[doc = "Returns response with indentations and line breaks."]
                                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                    self.pretty_print = Some(value);
                                    self
                                }
                                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                                pub fn quota_user(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.quota_user = Some(value.into());
                                    self
                                }
                                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                                pub fn upload_protocol(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_protocol = Some(value.into());
                                    self
                                }
                                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                                pub fn upload_type(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_type = Some(value.into());
                                    self
                                }
                                #[doc = "V1 error format."]
                                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                    self.xgafv = Some(value);
                                    self
                                }
                                pub fn execute<T>(
                                    mut self,
                                ) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    self._execute()
                                }
                                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                                pub fn execute_text(
                                    self,
                                ) -> Result<String, Box<dyn ::std::error::Error>>
                                {
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.text()?)
                                }
                                pub fn execute_debug(
                                    self,
                                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                                {
                                    self.execute()
                                }
                                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    if self.fields.is_none() {
                                        self.fields = Some(T::field_selector());
                                    }
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.json()?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://healthcare.googleapis.com/".to_owned();
                                    output.push_str("v1beta1/");
                                    output.push_str(&self.parent);
                                    output.push_str("/dicomWeb/");
                                    output.push_str(&self.dicom_web_path);
                                    output
                                }
                                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                                    let req = req.query(&[("access_token", &self.access_token)]);
                                    let req = req.query(&[("alt", &self.alt)]);
                                    let req = req.query(&[("callback", &self.callback)]);
                                    let req = req.query(&[("fields", &self.fields)]);
                                    let req = req.query(&[("key", &self.key)]);
                                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                                    let req =
                                        req.query(&[("upload_protocol", &self.upload_protocol)]);
                                    let req = req.query(&[("uploadType", &self.upload_type)]);
                                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                    let mut auth = self.auth.lock().unwrap();
                                    let req = req.bearer_auth(
                                        auth.token::<_, &str>(&[
                                            "https://www.googleapis.com/auth/cloud-platform",
                                        ])
                                        .unwrap()
                                        .access_token,
                                    );
                                    req
                                }
                            }
                            #[derive(Debug, Clone)]
                            pub struct RetrieveInstanceRequestBuilder<'a, A> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                parent: String,
                                dicom_web_path: String,
                                access_token: Option<String>,
                                alt: Option<crate::params::Alt>,
                                callback: Option<String>,
                                fields: Option<String>,
                                key: Option<String>,
                                oauth_token: Option<String>,
                                pretty_print: Option<bool>,
                                quota_user: Option<String>,
                                upload_protocol: Option<String>,
                                upload_type: Option<String>,
                                xgafv: Option<crate::params::Xgafv>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> RetrieveInstanceRequestBuilder<'a, A> {
                                #[doc = "OAuth access token."]
                                pub fn access_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.access_token = Some(value.into());
                                    self
                                }
                                #[doc = "Data format for response."]
                                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                    self.alt = Some(value);
                                    self
                                }
                                #[doc = "JSONP"]
                                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.callback = Some(value.into());
                                    self
                                }
                                #[doc = "Selector specifying which fields to include in a partial response."]
                                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.fields = Some(value.into());
                                    self
                                }
                                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.key = Some(value.into());
                                    self
                                }
                                #[doc = "OAuth 2.0 token for the current user."]
                                pub fn oauth_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.oauth_token = Some(value.into());
                                    self
                                }
                                #[doc = "Returns response with indentations and line breaks."]
                                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                    self.pretty_print = Some(value);
                                    self
                                }
                                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                                pub fn quota_user(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.quota_user = Some(value.into());
                                    self
                                }
                                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                                pub fn upload_protocol(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_protocol = Some(value.into());
                                    self
                                }
                                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                                pub fn upload_type(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_type = Some(value.into());
                                    self
                                }
                                #[doc = "V1 error format."]
                                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                    self.xgafv = Some(value);
                                    self
                                }
                                pub fn execute<T>(
                                    mut self,
                                ) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    self._execute()
                                }
                                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                                pub fn execute_text(
                                    self,
                                ) -> Result<String, Box<dyn ::std::error::Error>>
                                {
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.text()?)
                                }
                                pub fn execute_debug(
                                    self,
                                ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                                {
                                    self.execute()
                                }
                                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    if self.fields.is_none() {
                                        self.fields = Some(T::field_selector());
                                    }
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.json()?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://healthcare.googleapis.com/".to_owned();
                                    output.push_str("v1beta1/");
                                    output.push_str(&self.parent);
                                    output.push_str("/dicomWeb/");
                                    output.push_str(&self.dicom_web_path);
                                    output
                                }
                                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                                    let req = req.query(&[("access_token", &self.access_token)]);
                                    let req = req.query(&[("alt", &self.alt)]);
                                    let req = req.query(&[("callback", &self.callback)]);
                                    let req = req.query(&[("fields", &self.fields)]);
                                    let req = req.query(&[("key", &self.key)]);
                                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                                    let req =
                                        req.query(&[("upload_protocol", &self.upload_protocol)]);
                                    let req = req.query(&[("uploadType", &self.upload_type)]);
                                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                    let mut auth = self.auth.lock().unwrap();
                                    let req = req.bearer_auth(
                                        auth.token::<_, &str>(&[
                                            "https://www.googleapis.com/auth/cloud-platform",
                                        ])
                                        .unwrap()
                                        .access_token,
                                    );
                                    req
                                }
                            }
                            #[derive(Debug, Clone)]
                            pub struct RetrieveMetadataRequestBuilder<'a, A> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                parent: String,
                                dicom_web_path: String,
                                access_token: Option<String>,
                                alt: Option<crate::params::Alt>,
                                callback: Option<String>,
                                fields: Option<String>,
                                key: Option<String>,
                                oauth_token: Option<String>,
                                pretty_print: Option<bool>,
                                quota_user: Option<String>,
                                upload_protocol: Option<String>,
                                upload_type: Option<String>,
                                xgafv: Option<crate::params::Xgafv>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> RetrieveMetadataRequestBuilder<'a, A> {
                                #[doc = "OAuth access token."]
                                pub fn access_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.access_token = Some(value.into());
                                    self
                                }
                                #[doc = "Data format for response."]
                                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                    self.alt = Some(value);
                                    self
                                }
                                #[doc = "JSONP"]
                                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.callback = Some(value.into());
                                    self
                                }
                                #[doc = "Selector specifying which fields to include in a partial response."]
                                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.fields = Some(value.into());
                                    self
                                }
                                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.key = Some(value.into());
                                    self
                                }
                                #[doc = "OAuth 2.0 token for the current user."]
                                pub fn oauth_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.oauth_token = Some(value.into());
                                    self
                                }
                                #[doc = "Returns response with indentations and line breaks."]
                                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                    self.pretty_print = Some(value);
                                    self
                                }
                                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                                pub fn quota_user(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.quota_user = Some(value.into());
                                    self
                                }
                                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                                pub fn upload_protocol(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_protocol = Some(value.into());
                                    self
                                }
                                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                                pub fn upload_type(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_type = Some(value.into());
                                    self
                                }
                                #[doc = "V1 error format."]
                                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                    self.xgafv = Some(value);
                                    self
                                }
                                pub fn execute<T>(
                                    mut self,
                                ) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    self._execute()
                                }
                                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                                pub fn execute_text(
                                    self,
                                ) -> Result<String, Box<dyn ::std::error::Error>>
                                {
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.text()?)
                                }
                                pub fn execute_debug(
                                    self,
                                ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                                {
                                    self.execute()
                                }
                                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    if self.fields.is_none() {
                                        self.fields = Some(T::field_selector());
                                    }
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.json()?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://healthcare.googleapis.com/".to_owned();
                                    output.push_str("v1beta1/");
                                    output.push_str(&self.parent);
                                    output.push_str("/dicomWeb/");
                                    output.push_str(&self.dicom_web_path);
                                    output
                                }
                                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                                    let req = req.query(&[("access_token", &self.access_token)]);
                                    let req = req.query(&[("alt", &self.alt)]);
                                    let req = req.query(&[("callback", &self.callback)]);
                                    let req = req.query(&[("fields", &self.fields)]);
                                    let req = req.query(&[("key", &self.key)]);
                                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                                    let req =
                                        req.query(&[("upload_protocol", &self.upload_protocol)]);
                                    let req = req.query(&[("uploadType", &self.upload_type)]);
                                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                    let mut auth = self.auth.lock().unwrap();
                                    let req = req.bearer_auth(
                                        auth.token::<_, &str>(&[
                                            "https://www.googleapis.com/auth/cloud-platform",
                                        ])
                                        .unwrap()
                                        .access_token,
                                    );
                                    req
                                }
                            }
                            #[derive(Debug, Clone)]
                            pub struct RetrieveRenderedRequestBuilder<'a, A> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                parent: String,
                                dicom_web_path: String,
                                access_token: Option<String>,
                                alt: Option<crate::params::Alt>,
                                callback: Option<String>,
                                fields: Option<String>,
                                key: Option<String>,
                                oauth_token: Option<String>,
                                pretty_print: Option<bool>,
                                quota_user: Option<String>,
                                upload_protocol: Option<String>,
                                upload_type: Option<String>,
                                xgafv: Option<crate::params::Xgafv>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> RetrieveRenderedRequestBuilder<'a, A> {
                                #[doc = "OAuth access token."]
                                pub fn access_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.access_token = Some(value.into());
                                    self
                                }
                                #[doc = "Data format for response."]
                                pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                    self.alt = Some(value);
                                    self
                                }
                                #[doc = "JSONP"]
                                pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.callback = Some(value.into());
                                    self
                                }
                                #[doc = "Selector specifying which fields to include in a partial response."]
                                pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.fields = Some(value.into());
                                    self
                                }
                                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                                pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                    self.key = Some(value.into());
                                    self
                                }
                                #[doc = "OAuth 2.0 token for the current user."]
                                pub fn oauth_token(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.oauth_token = Some(value.into());
                                    self
                                }
                                #[doc = "Returns response with indentations and line breaks."]
                                pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                    self.pretty_print = Some(value);
                                    self
                                }
                                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                                pub fn quota_user(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.quota_user = Some(value.into());
                                    self
                                }
                                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                                pub fn upload_protocol(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_protocol = Some(value.into());
                                    self
                                }
                                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                                pub fn upload_type(
                                    &mut self,
                                    value: impl Into<String>,
                                ) -> &mut Self {
                                    self.upload_type = Some(value.into());
                                    self
                                }
                                #[doc = "V1 error format."]
                                pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                                    self.xgafv = Some(value);
                                    self
                                }
                                pub fn execute<T>(
                                    mut self,
                                ) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    self._execute()
                                }
                                #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                                pub fn execute_text(
                                    self,
                                ) -> Result<String, Box<dyn ::std::error::Error>>
                                {
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.text()?)
                                }
                                pub fn execute_debug(
                                    self,
                                ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                                {
                                    self.execute()
                                }
                                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    if self.fields.is_none() {
                                        self.fields = Some(T::field_selector());
                                    }
                                    let req = self._request(&self._path());
                                    Ok(req.send()?.error_for_status()?.json()?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://healthcare.googleapis.com/".to_owned();
                                    output.push_str("v1beta1/");
                                    output.push_str(&self.parent);
                                    output.push_str("/dicomWeb/");
                                    output.push_str(&self.dicom_web_path);
                                    output
                                }
                                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                                    let req = req.query(&[("access_token", &self.access_token)]);
                                    let req = req.query(&[("alt", &self.alt)]);
                                    let req = req.query(&[("callback", &self.callback)]);
                                    let req = req.query(&[("fields", &self.fields)]);
                                    let req = req.query(&[("key", &self.key)]);
                                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                                    let req =
                                        req.query(&[("upload_protocol", &self.upload_protocol)]);
                                    let req = req.query(&[("uploadType", &self.upload_type)]);
                                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                    let mut auth = self.auth.lock().unwrap();
                                    let req = req.bearer_auth(
                                        auth.token::<_, &str>(&[
                                            "https://www.googleapis.com/auth/cloud-platform",
                                        ])
                                        .unwrap()
                                        .access_token,
                                    );
                                    req
                                }
                            }
                            pub mod frames {
                                pub mod params {}
                                pub struct FramesActions<'a, A> {
                                    pub(super) reqwest: &'a reqwest::Client,
                                    pub(super) auth: &'a std::sync::Mutex<A>,
                                }
                                impl<'a, A: yup_oauth2::GetToken> FramesActions<'a, A> {
                                    #[doc = "RetrieveFrames returns instances associated with the given study, series,\nSOP Instance UID and frame numbers. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                                    pub fn retrieve_frames(
                                        &self,
                                        parent: impl Into<String>,
                                        dicom_web_path: impl Into<String>,
                                    ) -> RetrieveFramesRequestBuilder<A>
                                    {
                                        RetrieveFramesRequestBuilder {
                                            reqwest: &self.reqwest,
                                            auth: &self.auth,
                                            access_token: None,
                                            alt: None,
                                            callback: None,
                                            fields: None,
                                            key: None,
                                            oauth_token: None,
                                            pretty_print: None,
                                            quota_user: None,
                                            upload_protocol: None,
                                            upload_type: None,
                                            xgafv: None,
                                            parent: parent.into(),
                                            dicom_web_path: dicom_web_path.into(),
                                        }
                                    }
                                    #[doc = "RetrieveRenderedFrames returns instances associated with the given study,\nseries, SOP Instance UID and frame numbers in an acceptable Rendered Media\nType. See\nhttp://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4."]
                                    pub fn retrieve_rendered(
                                        &self,
                                        parent: impl Into<String>,
                                        dicom_web_path: impl Into<String>,
                                    ) -> RetrieveRenderedRequestBuilder<A>
                                    {
                                        RetrieveRenderedRequestBuilder {
                                            reqwest: &self.reqwest,
                                            auth: &self.auth,
                                            access_token: None,
                                            alt: None,
                                            callback: None,
                                            fields: None,
                                            key: None,
                                            oauth_token: None,
                                            pretty_print: None,
                                            quota_user: None,
                                            upload_protocol: None,
                                            upload_type: None,
                                            xgafv: None,
                                            parent: parent.into(),
                                            dicom_web_path: dicom_web_path.into(),
                                        }
                                    }
                                }
                                #[derive(Debug, Clone)]
                                pub struct RetrieveFramesRequestBuilder<'a, A> {
                                    pub(crate) reqwest: &'a ::reqwest::Client,
                                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                    parent: String,
                                    dicom_web_path: String,
                                    access_token: Option<String>,
                                    alt: Option<crate::params::Alt>,
                                    callback: Option<String>,
                                    fields: Option<String>,
                                    key: Option<String>,
                                    oauth_token: Option<String>,
                                    pretty_print: Option<bool>,
                                    quota_user: Option<String>,
                                    upload_protocol: Option<String>,
                                    upload_type: Option<String>,
                                    xgafv: Option<crate::params::Xgafv>,
                                }
                                impl<'a, A: yup_oauth2::GetToken> RetrieveFramesRequestBuilder<'a, A> {
                                    #[doc = "OAuth access token."]
                                    pub fn access_token(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.access_token = Some(value.into());
                                        self
                                    }
                                    #[doc = "Data format for response."]
                                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                        self.alt = Some(value);
                                        self
                                    }
                                    #[doc = "JSONP"]
                                    pub fn callback(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.callback = Some(value.into());
                                        self
                                    }
                                    #[doc = "Selector specifying which fields to include in a partial response."]
                                    pub fn fields(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.fields = Some(value.into());
                                        self
                                    }
                                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                        self.key = Some(value.into());
                                        self
                                    }
                                    #[doc = "OAuth 2.0 token for the current user."]
                                    pub fn oauth_token(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.oauth_token = Some(value.into());
                                        self
                                    }
                                    #[doc = "Returns response with indentations and line breaks."]
                                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                        self.pretty_print = Some(value);
                                        self
                                    }
                                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                                    pub fn quota_user(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.quota_user = Some(value.into());
                                        self
                                    }
                                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                                    pub fn upload_protocol(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.upload_protocol = Some(value.into());
                                        self
                                    }
                                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                                    pub fn upload_type(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.upload_type = Some(value.into());
                                        self
                                    }
                                    #[doc = "V1 error format."]
                                    pub fn xgafv(
                                        &mut self,
                                        value: crate::params::Xgafv,
                                    ) -> &mut Self {
                                        self.xgafv = Some(value);
                                        self
                                    }
                                    pub fn execute<T>(
                                        mut self,
                                    ) -> Result<T, Box<dyn ::std::error::Error>>
                                    where
                                        T: ::serde::de::DeserializeOwned
                                            + ::field_selector::FieldSelector,
                                    {
                                        self._execute()
                                    }
                                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                                    pub fn execute_text(
                                        self,
                                    ) -> Result<String, Box<dyn ::std::error::Error>>
                                    {
                                        let req = self._request(&self._path());
                                        Ok(req.send()?.error_for_status()?.text()?)
                                    }
                                    pub fn execute_debug(
                                        self,
                                    ) -> Result<
                                        crate::schemas::HttpBody,
                                        Box<dyn ::std::error::Error>,
                                    > {
                                        self.execute()
                                    }
                                    fn _execute<T>(
                                        &mut self,
                                    ) -> Result<T, Box<dyn ::std::error::Error>>
                                    where
                                        T: ::serde::de::DeserializeOwned
                                            + ::field_selector::FieldSelector,
                                    {
                                        if self.fields.is_none() {
                                            self.fields = Some(T::field_selector());
                                        }
                                        let req = self._request(&self._path());
                                        Ok(req.send()?.error_for_status()?.json()?)
                                    }
                                    fn _path(&self) -> String {
                                        let mut output =
                                            "https://healthcare.googleapis.com/".to_owned();
                                        output.push_str("v1beta1/");
                                        output.push_str(&self.parent);
                                        output.push_str("/dicomWeb/");
                                        output.push_str(&self.dicom_web_path);
                                        output
                                    }
                                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                        let req =
                                            self.reqwest.request(::reqwest::Method::GET, path);
                                        let req =
                                            req.query(&[("access_token", &self.access_token)]);
                                        let req = req.query(&[("alt", &self.alt)]);
                                        let req = req.query(&[("callback", &self.callback)]);
                                        let req = req.query(&[("fields", &self.fields)]);
                                        let req = req.query(&[("key", &self.key)]);
                                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                                        let req = req
                                            .query(&[("upload_protocol", &self.upload_protocol)]);
                                        let req = req.query(&[("uploadType", &self.upload_type)]);
                                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                        let mut auth = self.auth.lock().unwrap();
                                        let req = req.bearer_auth(
                                            auth.token::<_, &str>(&[
                                                "https://www.googleapis.com/auth/cloud-platform",
                                            ])
                                            .unwrap()
                                            .access_token,
                                        );
                                        req
                                    }
                                }
                                #[derive(Debug, Clone)]
                                pub struct RetrieveRenderedRequestBuilder<'a, A> {
                                    pub(crate) reqwest: &'a ::reqwest::Client,
                                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                    parent: String,
                                    dicom_web_path: String,
                                    access_token: Option<String>,
                                    alt: Option<crate::params::Alt>,
                                    callback: Option<String>,
                                    fields: Option<String>,
                                    key: Option<String>,
                                    oauth_token: Option<String>,
                                    pretty_print: Option<bool>,
                                    quota_user: Option<String>,
                                    upload_protocol: Option<String>,
                                    upload_type: Option<String>,
                                    xgafv: Option<crate::params::Xgafv>,
                                }
                                impl<'a, A: yup_oauth2::GetToken> RetrieveRenderedRequestBuilder<'a, A> {
                                    #[doc = "OAuth access token."]
                                    pub fn access_token(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.access_token = Some(value.into());
                                        self
                                    }
                                    #[doc = "Data format for response."]
                                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                                        self.alt = Some(value);
                                        self
                                    }
                                    #[doc = "JSONP"]
                                    pub fn callback(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.callback = Some(value.into());
                                        self
                                    }
                                    #[doc = "Selector specifying which fields to include in a partial response."]
                                    pub fn fields(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.fields = Some(value.into());
                                        self
                                    }
                                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                                        self.key = Some(value.into());
                                        self
                                    }
                                    #[doc = "OAuth 2.0 token for the current user."]
                                    pub fn oauth_token(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.oauth_token = Some(value.into());
                                        self
                                    }
                                    #[doc = "Returns response with indentations and line breaks."]
                                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                                        self.pretty_print = Some(value);
                                        self
                                    }
                                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                                    pub fn quota_user(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.quota_user = Some(value.into());
                                        self
                                    }
                                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                                    pub fn upload_protocol(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.upload_protocol = Some(value.into());
                                        self
                                    }
                                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                                    pub fn upload_type(
                                        &mut self,
                                        value: impl Into<String>,
                                    ) -> &mut Self {
                                        self.upload_type = Some(value.into());
                                        self
                                    }
                                    #[doc = "V1 error format."]
                                    pub fn xgafv(
                                        &mut self,
                                        value: crate::params::Xgafv,
                                    ) -> &mut Self {
                                        self.xgafv = Some(value);
                                        self
                                    }
                                    pub fn execute<T>(
                                        mut self,
                                    ) -> Result<T, Box<dyn ::std::error::Error>>
                                    where
                                        T: ::serde::de::DeserializeOwned
                                            + ::field_selector::FieldSelector,
                                    {
                                        self._execute()
                                    }
                                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                                    pub fn execute_text(
                                        self,
                                    ) -> Result<String, Box<dyn ::std::error::Error>>
                                    {
                                        let req = self._request(&self._path());
                                        Ok(req.send()?.error_for_status()?.text()?)
                                    }
                                    pub fn execute_debug(
                                        self,
                                    ) -> Result<
                                        crate::schemas::HttpBody,
                                        Box<dyn ::std::error::Error>,
                                    > {
                                        self.execute()
                                    }
                                    fn _execute<T>(
                                        &mut self,
                                    ) -> Result<T, Box<dyn ::std::error::Error>>
                                    where
                                        T: ::serde::de::DeserializeOwned
                                            + ::field_selector::FieldSelector,
                                    {
                                        if self.fields.is_none() {
                                            self.fields = Some(T::field_selector());
                                        }
                                        let req = self._request(&self._path());
                                        Ok(req.send()?.error_for_status()?.json()?)
                                    }
                                    fn _path(&self) -> String {
                                        let mut output =
                                            "https://healthcare.googleapis.com/".to_owned();
                                        output.push_str("v1beta1/");
                                        output.push_str(&self.parent);
                                        output.push_str("/dicomWeb/");
                                        output.push_str(&self.dicom_web_path);
                                        output
                                    }
                                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                        let req =
                                            self.reqwest.request(::reqwest::Method::GET, path);
                                        let req =
                                            req.query(&[("access_token", &self.access_token)]);
                                        let req = req.query(&[("alt", &self.alt)]);
                                        let req = req.query(&[("callback", &self.callback)]);
                                        let req = req.query(&[("fields", &self.fields)]);
                                        let req = req.query(&[("key", &self.key)]);
                                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                                        let req = req
                                            .query(&[("upload_protocol", &self.upload_protocol)]);
                                        let req = req.query(&[("uploadType", &self.upload_type)]);
                                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                                        let mut auth = self.auth.lock().unwrap();
                                        let req = req.bearer_auth(
                                            auth.token::<_, &str>(&[
                                                "https://www.googleapis.com/auth/cloud-platform",
                                            ])
                                            .unwrap()
                                            .access_token,
                                        );
                                        req
                                    }
                                }
                            }
                        }
                    }
                }
            }
            pub mod fhir_stores {
                pub mod params {}
                pub struct FhirStoresActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FhirStoresActions<'a, A> {
                    #[doc = "Creates a new FHIR store within the parent dataset."]
                    pub fn create(
                        &self,
                        request: crate::schemas::FhirStore,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            fhir_store_id: None,
                        }
                    }
                    #[doc = "Deletes the specified FHIR store and removes all resources within it."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Export resources from the FHIR store to the specified destination.\n\nThis method returns an Operation that can\nbe used to track the status of the export by calling\nGetOperation.\n\nImmediate fatal errors appear in the\nerror field.\nOtherwise, when the operation finishes, a detailed response of type\nExportResourcesResponse is returned in the\nresponse field.\nThe metadata field type for this\noperation is OperationMetadata."]
                    pub fn export(
                        &self,
                        request: crate::schemas::ExportResourcesRequest,
                        name: impl Into<String>,
                    ) -> ExportRequestBuilder<A> {
                        ExportRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the configuration of the specified FHIR store."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder<A> {
                        GetIamPolicyRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Import resources to the FHIR store by loading data from the specified\nsources. This method is optimized to load large quantities of data using\nimport semantics that ignore some FHIR store configuration options and are\nnot suitable for all use cases. It is primarily intended to load data into\nan empty FHIR store that is not being used by other clients. In cases\nwhere this method is not appropriate, consider using ExecuteBundle to\nload data.\n\nEvery resource in the input must contain a client-supplied ID, and will be\nstored using that ID regardless of the\nenable_update_create setting on the FHIR\nstore.\n\nThe import process does not enforce referential integrity, regardless of\nthe\ndisable_referential_integrity\nsetting on the FHIR store. This allows the import of resources with\narbitrary interdependencies without considering grouping or ordering, but\nif the input data contains invalid references or if some resources fail to\nbe imported, the FHIR store might be left in a state that violates\nreferential integrity.\n\nIf a resource with the specified ID already exists, the most recent\nversion of the resource is overwritten without creating a new historical\nversion, regardless of the\ndisable_resource_versioning\nsetting on the FHIR store. If transient failures occur during the import,\nit is possible that successfully imported resources will be overwritten\nmore than once.\n\nThe import operation is idempotent unless the input data contains multiple\nvalid resources with the same ID but different contents. In that case,\nafter the import completes, the store will contain exactly one resource\nwith that ID but there is no ordering guarantee on which version of the\ncontents it will have. The operation result counters do not count\nduplicate IDs as an error and will count one success for each resource in\nthe input, which might result in a success count larger than the number\nof resources in the FHIR store. This often occurs when importing data\norganized in bundles produced by Patient-everything\nwhere each bundle contains its own copy of a resource such as Practitioner\nthat might be referred to by many patients.\n\nIf some resources fail to import, for example due to parsing errors,\nsuccessfully imported resources are not rolled back.\n\nThe location and format of the input data is specified by the parameters\nbelow. Note that if no format is specified, this method assumes the\n`BUNDLE` format. When using the `BUNDLE` format this method ignores the\n`Bundle.type` field, except for the special case of `history`, and does\nnot apply any of the bundle processing semantics for batch or transaction\nbundles. Unlike in ExecuteBundle, transaction bundles are not executed\nas a single transaction and bundle-internal references are not rewritten.\nThe bundle is treated as a collection of resources to be written as\nprovided in `Bundle.entry.resource`, ignoring `Bundle.entry.request`. As\nan example, this allows the import of `searchset` bundles produced by a\nFHIR search or\nPatient-everything operation.\n\nIf history imports are enabled by setting\nenable_history_import in the FHIR\nstore's configuration, this method can import historical versions\nof a resource by supplying a bundle of type `history` and using the\n`BUNDLE` format. The historical versions in the bundle must have\n`lastUpdated` timestamps, and the resulting resource history on the server\nwill appear as if the versions had been created at those timestamps. If a\ncurrent or historical version with the supplied resource ID already\nexists, the bundle is rejected to avoid creating an inconsistent sequence\nof resource versions.\n\nThis method returns an Operation that can\nbe used to track the status of the import by calling\nGetOperation.\n\nImmediate fatal errors appear in the\nerror field.\nOtherwise, when the operation finishes, a detailed response of type\nImportResourcesResponse is returned in the\nresponse field.\nThe metadata field type for this\noperation is OperationMetadata."]
                    pub fn import(
                        &self,
                        request: crate::schemas::ImportResourcesRequest,
                        name: impl Into<String>,
                    ) -> ImportRequestBuilder<A> {
                        ImportRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists the FHIR stores in the given dataset."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the configuration of the specified FHIR store."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::FhirStore,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder<A> {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                    #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy."]
                    pub fn set_iam_policy(
                        &self,
                        request: crate::schemas::SetIamPolicyRequest,
                        resource: impl Into<String>,
                    ) -> SetIamPolicyRequestBuilder<A> {
                        SetIamPolicyRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                    pub fn test_iam_permissions(
                        &self,
                        request: crate::schemas::TestIamPermissionsRequest,
                        resource: impl Into<String>,
                    ) -> TestIamPermissionsRequestBuilder<A> {
                        TestIamPermissionsRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the fhir resource"]
                    pub fn fhir(&self) -> fhir::FhirActions<A> {
                        fhir::FhirActions
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::FhirStore,
                    parent: String,
                    fhir_store_id: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "The ID of the FHIR store that is being created.\nThe string must match the following regex: `[\\p{L}\\p{N}_\\-\\.]{1,256}`."]
                    pub fn fhir_store_id(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fhir_store_id = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::FhirStore, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/fhirStores");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fhirStoreId", &self.fhir_store_id)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ExportRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::ExportResourcesRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ExportRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output.push_str(":export");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::FhirStore, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    resource: String,
                    options_requested_policy_version: Option<i32>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetIamPolicyRequestBuilder<'a, A> {
                    #[doc = "Optional. The policy format version to be returned.\nAcceptable values are 0 and 1.\nIf the value is 0, or the field is omitted, policy format version 1 will be\nreturned."]
                    pub fn options_requested_policy_version(&mut self, value: i32) -> &mut Self {
                        self.options_requested_policy_version = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":getIamPolicy");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ImportRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::ImportResourcesRequest,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ImportRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output.push_str(":import");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Restricts stores returned to those matching a filter. Syntax:\nhttps://cloud.google.com/appengine/docs/standard/python/search/query_strings\nOnly filtering on labels is supported, for example `labels.key=value`."]
                    pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Limit on the number of FHIR stores to return in a single response.  If zero\nthe default page size of 100 is used."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from the previous List request, if any."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_fhir_stores<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "fhirStores")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListFhirStoresResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/fhirStores");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::FhirStore,
                    name: String,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                    #[doc = "The update mask applies to the resource. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
                    pub fn update_mask(&mut self, value: impl Into<String>) -> &mut Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::FhirStore, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SetIamPolicyRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SetIamPolicyRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":setIamPolicy");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct TestIamPermissionsRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> TestIamPermissionsRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::TestIamPermissionsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":testIamPermissions");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                pub mod fhir {
                    pub mod params {}
                    pub struct FhirActions<'a, A> {
                        pub(super) reqwest: &'a reqwest::Client,
                        pub(super) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> FhirActions<'a, A> {
                        #[doc = "Retrieves the N most recent `Observation` resources for a subject matching\nsearch criteria specified as query parameters, grouped by\n`Observation.code`, sorted from most recent to oldest.\n\nImplements the FHIR extended operation\n[Observation-lastn](http://hl7.org/implement/standards/fhir/STU3/observation-operations.html#lastn).\n\nSearch terms are provided as query parameters following the same pattern as\nthe search method. This operation accepts an additional\nquery parameter `max`, which specifies N, the maximum number of\nObservations to return from each group, with a default of 1.\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `searchset`, containing the results of the\noperation.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn observation_lastn(
                            &self,
                            parent: impl Into<String>,
                        ) -> ObservationLastnRequestBuilder<A> {
                            ObservationLastnRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Retrieves all the resources in the patient compartment for a `Patient`\nresource.\n\nImplements the FHIR extended operation\n[Patient-everything](http://hl7.org/implement/standards/fhir/STU3/patient-operations.html#everything).\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `searchset`, containing the results of the\noperation.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn patient_everything(
                            &self,
                            name: impl Into<String>,
                        ) -> PatientEverythingRequestBuilder<A> {
                            PatientEverythingRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                end: None,
                                start: None,
                            }
                        }
                        #[doc = "Deletes all the historical versions of a resource (excluding the current\nversion) from the FHIR store. To remove all versions of a resource, first\ndelete the current version and then call this method.\n\nThis is not a FHIR standard operation."]
                        pub fn resource_purge(
                            &self,
                            name: impl Into<String>,
                        ) -> ResourcePurgeRequestBuilder<A> {
                            ResourcePurgeRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets the FHIR [capability\nstatement](http://hl7.org/implement/standards/fhir/STU3/capabilitystatement.html)\nfor the store, which contains a description of functionality supported by\nthe server.\n\nImplements the FHIR standard [capabilities\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#capabilities).\n\nOn success, the response body will contain a JSON-encoded representation\nof a `CapabilityStatement` resource."]
                        pub fn capabilities(
                            &self,
                            name: impl Into<String>,
                        ) -> CapabilitiesRequestBuilder<A> {
                            CapabilitiesRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Deletes FHIR resources that match a search query.\n\nImplements the FHIR standard [conditional delete\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.13.1).\nIf multiple resources match, all of them will be deleted.\n\nSearch terms are provided as query parameters following the same pattern as\nthe search method.\n\nNote: Unless resource versioning is disabled by setting the\ndisable_resource_versioning flag\non the FHIR store, the deleted resources will be moved to a history\nrepository that can still be retrieved through vread\nand related methods, unless they are removed by the\npurge method."]
                        pub fn conditional_delete(
                            &self,
                            parent: impl Into<String>,
                            r#type: impl Into<String>,
                        ) -> ConditionalDeleteRequestBuilder<A> {
                            ConditionalDeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                r#type: r#type.into(),
                            }
                        }
                        #[doc = "If a resource is found based on the search criteria specified in the query\nparameters, updates part of that resource by applying the operations\nspecified in a [JSON Patch](http://jsonpatch.com/) document.\n\nImplements the FHIR standard [conditional patch\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#patch).\n\nSearch terms are provided as query parameters following the same pattern as\nthe search method.\n\nIf the search criteria identify more than one match, the request will\nreturn a `412 Precondition Failed` error.\n\nThe request body must contain a JSON Patch document, and the request\nheaders must contain `Content-Type: application/json-patch+json`.\n\nOn success, the response body will contain a JSON-encoded representation\nof the updated resource, including the server-assigned version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn conditional_patch(
                            &self,
                            request: crate::schemas::HttpBody,
                            parent: impl Into<String>,
                            r#type: impl Into<String>,
                        ) -> ConditionalPatchRequestBuilder<A> {
                            ConditionalPatchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                r#type: r#type.into(),
                            }
                        }
                        #[doc = "If a resource is found based on the search criteria specified in the query\nparameters, updates the entire contents of that resource.\n\nImplements the FHIR standard [conditional update\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#cond-update).\n\nSearch terms are provided as query parameters following the same pattern as\nthe search method.\n\nIf the search criteria identify more than one match, the request will\nreturn a `412 Precondition Failed` error.\nIf the search criteria identify zero matches, and the supplied resource\nbody contains an `id`, and the FHIR store has\nenable_update_create set, creates the\nresource with the client-specified ID. If the search criteria identify zero\nmatches, and the supplied resource body does not contain an `id`, the\nresource will be created with a server-assigned ID as per the\ncreate method.\n\nThe request body must contain a JSON-encoded FHIR resource, and the request\nheaders must contain `Content-Type: application/fhir+json`.\n\nOn success, the response body will contain a JSON-encoded representation\nof the updated resource, including the server-assigned version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn conditional_update(
                            &self,
                            request: crate::schemas::HttpBody,
                            parent: impl Into<String>,
                            r#type: impl Into<String>,
                        ) -> ConditionalUpdateRequestBuilder<A> {
                            ConditionalUpdateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                r#type: r#type.into(),
                            }
                        }
                        #[doc = "Creates a FHIR resource.\n\nImplements the FHIR standard [create\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#create),\nwhich creates a new resource with a server-assigned resource ID.\n\nAlso supports the FHIR standard [conditional create\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#ccreate),\nspecified by supplying an `If-None-Exist` header containing a FHIR search\nquery. If no resources match this search query, the server processes the\ncreate operation as normal.\n\nThe request body must contain a JSON-encoded FHIR resource, and the request\nheaders must contain `Content-Type: application/fhir+json`.\n\nOn success, the response body will contain a JSON-encoded representation\nof the resource as it was created on the server, including the\nserver-assigned resource ID and version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn create(
                            &self,
                            request: crate::schemas::HttpBody,
                            parent: impl Into<String>,
                            r#type: impl Into<String>,
                        ) -> CreateRequestBuilder<A> {
                            CreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                r#type: r#type.into(),
                            }
                        }
                        #[doc = "Deletes a FHIR resource.\n\nImplements the FHIR standard [delete\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#delete).\n\nNote: Unless resource versioning is disabled by setting the\ndisable_resource_versioning flag\non the FHIR store, the deleted resources will be moved to a history\nrepository that can still be retrieved through vread\nand related methods, unless they are removed by the\npurge method."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Executes all the requests in the given Bundle.\n\nImplements the FHIR standard [batch/transaction\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#transaction).\n\nSupports all interactions within a bundle, except search. This method\naccepts Bundles of type `batch` and `transaction`, processing them\naccording to the [batch processing\nrules](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.1)\nand [transaction processing\nrules](http://hl7.org/implement/standards/fhir/STU3/http.html#2.21.0.17.2).\n\nThe request body must contain a JSON-encoded FHIR `Bundle` resource, and\nthe request headers must contain `Content-Type: application/fhir+json`.\n\nFor a batch bundle or a successful transaction the response body will\ncontain a JSON-encoded representation of a `Bundle` resource of type\n`batch-response` or `transaction-response` containing one entry for each\nentry in the request, with the outcome of processing the entry. In the\ncase of an error for a transaction bundle, the response body will contain\na JSON-encoded `OperationOutcome` resource describing the reason for the\nerror. If the request cannot be mapped to a valid API method on a FHIR\nstore, a generic GCP error might be returned instead."]
                        pub fn execute_bundle(
                            &self,
                            request: crate::schemas::HttpBody,
                            parent: impl Into<String>,
                        ) -> ExecuteBundleRequestBuilder<A> {
                            ExecuteBundleRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Lists all the versions of a resource (including the current version and\ndeleted versions) from the FHIR store.\n\nImplements the per-resource form of the FHIR standard [history\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#history).\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `history`, containing the version history\nsorted from most recent to oldest versions.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn history(&self, name: impl Into<String>) -> HistoryRequestBuilder<A> {
                            HistoryRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                at: None,
                                count: None,
                                page: None,
                                since: None,
                            }
                        }
                        #[doc = "Updates part of an existing resource by applying the operations specified\nin a [JSON Patch](http://jsonpatch.com/) document.\n\nImplements the FHIR standard [patch\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#patch).\n\nThe request body must contain a JSON Patch document, and the request\nheaders must contain `Content-Type: application/json-patch+json`.\n\nOn success, the response body will contain a JSON-encoded representation\nof the updated resource, including the server-assigned version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::HttpBody,
                            name: impl Into<String>,
                        ) -> PatchRequestBuilder<A> {
                            PatchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets the contents of a FHIR resource.\n\nImplements the FHIR standard [read\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#read).\n\nAlso supports the FHIR standard [conditional read\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#cread)\nspecified by supplying an `If-Modified-Since` header with a date/time value\nor an `If-None-Match` header with an ETag value.\n\nOn success, the response body will contain a JSON-encoded representation\nof the resource.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn read(&self, name: impl Into<String>) -> ReadRequestBuilder<A> {
                            ReadRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Searches for resources in the given FHIR store according to criteria\nspecified as query parameters.\n\nImplements the FHIR standard [search\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#search)\nusing the search semantics described in the [FHIR Search\nspecification](http://hl7.org/implement/standards/fhir/STU3/search.html).\n\nSupports three methods of search defined by the specification:\n\n*  `GET [base]?[parameters]` to search across all resources.\n*  `GET [base]/[type]?[parameters]` to search resources of a specified\ntype.\n*  `POST [base]/[type]/_search?[parameters]` as an alternate form having\nthe same semantics as the `GET` method.\n\nThe `GET` methods do not support compartment searches. The `POST` method\ndoes not support `application/x-www-form-urlencoded` search parameters.\n\nOn success, the response body will contain a JSON-encoded representation\nof a `Bundle` resource of type `searchset`, containing the results of the\nsearch.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead.\n\nThe server's capability statement, retrieved through\ncapabilities, indicates what search parameters\nare supported on each FHIR resource. A list of all search parameters\ndefined by the specification can be found in the [FHIR Search Parameter\nRegistry](http://hl7.org/implement/standards/fhir/STU3/searchparameter-registry.html).\n\nSupported search modifiers: `:missing`, `:exact`, `:contains`, `:text`,\n`:in`, `:not-in`, `:above`, `:below`, `:[type]`, `:not`, and `:recurse`.\n\nSupported search result parameters: `_sort`, `_count`, `_include`,\n`_revinclude`, `_summary=text`, `_summary=data`, and `_elements`.\n\nThe maximum number of search results returned defaults to 100, which can\nbe overridden by the `_count` parameter up to a maximum limit of 1000. If\nthere are additional results, the returned `Bundle` will contain\npagination links.\n\nResources with a total size larger than 5MB or a field count larger than\n50,000 might not be fully searchable as the server might trim its generated\nsearch index in those cases.\n\nNote: FHIR resources are indexed asynchronously, so there might be a slight\ndelay between the time a resource is created or changes and when the change\nis reflected in search results."]
                        pub fn search(
                            &self,
                            request: crate::schemas::SearchResourcesRequest,
                            parent: impl Into<String>,
                        ) -> SearchRequestBuilder<A> {
                            SearchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Updates the entire contents of a resource.\n\nImplements the FHIR standard [update\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#update).\n\nIf the specified resource does\nnot exist and the FHIR store has\nenable_update_create set, creates the\nresource with the client-specified ID.\n\nThe request body must contain a JSON-encoded FHIR resource, and the request\nheaders must contain `Content-Type: application/fhir+json`. The resource\nmust contain an `id` element having an identical value to the ID in the\nREST path of the request.\n\nOn success, the response body will contain a JSON-encoded representation\nof the updated resource, including the server-assigned version ID.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn update(
                            &self,
                            request: crate::schemas::HttpBody,
                            name: impl Into<String>,
                        ) -> UpdateRequestBuilder<A> {
                            UpdateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets the contents of a version (current or historical) of a FHIR resource\nby version ID.\n\nImplements the FHIR standard [vread\ninteraction](http://hl7.org/implement/standards/fhir/STU3/http.html#vread).\n\nOn success, the response body will contain a JSON-encoded representation\nof the resource.\nErrors generated by the FHIR store will contain a JSON-encoded\n`OperationOutcome` resource describing the reason for the error. If the\nrequest cannot be mapped to a valid API method on a FHIR store, a generic\nGCP error might be returned instead."]
                        pub fn vread(&self, name: impl Into<String>) -> VreadRequestBuilder<A> {
                            VreadRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ObservationLastnRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ObservationLastnRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir/Observation/$lastn");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PatientEverythingRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        end: Option<String>,
                        start: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> PatientEverythingRequestBuilder<'a, A> {
                        #[doc = "The response includes records prior to the end date. If no end date is\nprovided, all records subsequent to the start date are in scope."]
                        pub fn end(&mut self, value: impl Into<String>) -> &mut Self {
                            self.end = Some(value.into());
                            self
                        }
                        #[doc = "The response includes records subsequent to the start date. If no start\ndate is provided, all records prior to the end date are in scope."]
                        pub fn start(&mut self, value: impl Into<String>) -> &mut Self {
                            self.start = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output.push_str("/$everything");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("end", &self.end)]);
                            let req = req.query(&[("start", &self.start)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ResourcePurgeRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ResourcePurgeRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output.push_str("/$purge");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct CapabilitiesRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> CapabilitiesRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output.push_str("/fhir/metadata");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ConditionalDeleteRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        r#type: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ConditionalDeleteRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir/");
                            output.push_str(&self.r#type);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ConditionalPatchRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        parent: String,
                        r#type: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ConditionalPatchRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir/");
                            output.push_str(&self.r#type);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ConditionalUpdateRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        parent: String,
                        r#type: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ConditionalUpdateRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir/");
                            output.push_str(&self.r#type);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::PUT, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        parent: String,
                        r#type: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir/");
                            output.push_str(&self.r#type);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ExecuteBundleRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ExecuteBundleRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct HistoryRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        at: Option<String>,
                        count: Option<i32>,
                        page: Option<String>,
                        since: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> HistoryRequestBuilder<'a, A> {
                        #[doc = "Only include resource versions that were current at some point during the\ntime period specified in the date time value. The date parameter format is\nyyyy-mm-ddThh:mm:ss[Z|(+|-)hh:mm]\n\nClients may specify any of the following:\n\n*  An entire year: `_at=2019`\n*  An entire month: `_at=2019-01`\n*  A specific day: `_at=2019-01-20`\n*  A specific second: `_at=2018-12-31T23:59:58Z`"]
                        pub fn at(&mut self, value: impl Into<String>) -> &mut Self {
                            self.at = Some(value.into());
                            self
                        }
                        #[doc = "The maximum number of search results on a page. Defaults to 1000."]
                        pub fn count(&mut self, value: i32) -> &mut Self {
                            self.count = Some(value);
                            self
                        }
                        #[doc = "Used to retrieve the first, previous, next, or last page of resource\nversions when using pagination. Value should be set to the value of the\n`link.url` field returned in the response to the previous request, where\n`link.relation` is \"first\", \"previous\", \"next\" or \"last\".\n\nOmit `page` if no previous request has been made."]
                        pub fn page(&mut self, value: impl Into<String>) -> &mut Self {
                            self.page = Some(value.into());
                            self
                        }
                        #[doc = "Only include resource versions that were created at or after the given\ninstant in time. The instant in time uses the format\nYYYY-MM-DDThh:mm:ss.sss+zz:zz (for example 2015-02-07T13:28:17.239+02:00 or\n2017-01-01T00:00:00Z). The time must be specified to the second and\ninclude a time zone."]
                        pub fn since(&mut self, value: impl Into<String>) -> &mut Self {
                            self.since = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output.push_str("/_history");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("at", &self.at)]);
                            let req = req.query(&[("count", &self.count)]);
                            let req = req.query(&[("page", &self.page)]);
                            let req = req.query(&[("since", &self.since)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ReadRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ReadRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct SearchRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::SearchResourcesRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/fhir/_search");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct UpdateRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::HttpBody,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::PUT, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct VreadRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> VreadRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::HttpBody, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                }
            }
            pub mod hl_7v2_stores {
                pub mod params {}
                pub struct Hl7V2StoresActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> Hl7V2StoresActions<'a, A> {
                    #[doc = "Creates a new HL7v2 store within the parent dataset."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Hl7V2Store,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            hl_7v2_store_id: None,
                        }
                    }
                    #[doc = "Deletes the specified HL7v2 store and removes all messages that are\ncontained within it."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the specified HL7v2 store."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                    pub fn get_iam_policy(
                        &self,
                        resource: impl Into<String>,
                    ) -> GetIamPolicyRequestBuilder<A> {
                        GetIamPolicyRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                            options_requested_policy_version: None,
                        }
                    }
                    #[doc = "Lists the HL7v2 stores in the given dataset."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the HL7v2 store."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::Hl7V2Store,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder<A> {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                    #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy."]
                    pub fn set_iam_policy(
                        &self,
                        request: crate::schemas::SetIamPolicyRequest,
                        resource: impl Into<String>,
                    ) -> SetIamPolicyRequestBuilder<A> {
                        SetIamPolicyRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                    pub fn test_iam_permissions(
                        &self,
                        request: crate::schemas::TestIamPermissionsRequest,
                        resource: impl Into<String>,
                    ) -> TestIamPermissionsRequestBuilder<A> {
                        TestIamPermissionsRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            resource: resource.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the messages resource"]
                    pub fn messages(&self) -> messages::MessagesActions<A> {
                        messages::MessagesActions
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Hl7V2Store,
                    parent: String,
                    hl_7v2_store_id: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "The ID of the HL7v2 store that is being created.\nThe string must match the following regex: `[\\p{L}\\p{N}_\\-\\.]{1,256}`."]
                    pub fn hl_7v_2_store_id(&mut self, value: impl Into<String>) -> &mut Self {
                        self.hl_7v2_store_id = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Hl7V2Store, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/hl7V2Stores");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("hl7V2StoreId", &self.hl_7v2_store_id)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Hl7V2Store, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetIamPolicyRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    resource: String,
                    options_requested_policy_version: Option<i32>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetIamPolicyRequestBuilder<'a, A> {
                    #[doc = "Optional. The policy format version to be returned.\nAcceptable values are 0 and 1.\nIf the value is 0, or the field is omitted, policy format version 1 will be\nreturned."]
                    pub fn options_requested_policy_version(&mut self, value: i32) -> &mut Self {
                        self.options_requested_policy_version = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":getIamPolicy");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[(
                            "options.requestedPolicyVersion",
                            &self.options_requested_policy_version,
                        )]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Restricts stores returned to those matching a filter. Syntax:\nhttps://cloud.google.com/appengine/docs/standard/python/search/query_strings\nOnly filtering on labels is supported, for example `labels.key=value`."]
                    pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Limit on the number of HL7v2 stores to return in a single response.\nIf zero the default page size of 100 is used."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from the previous List request, if any."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_hl_7v2_stores<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "hl7V2Stores")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListHl7V2StoresResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.parent);
                        output.push_str("/hl7V2Stores");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Hl7V2Store,
                    name: String,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                    #[doc = "The update mask applies to the resource. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
                    pub fn update_mask(&mut self, value: impl Into<String>) -> &mut Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Hl7V2Store, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SetIamPolicyRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SetIamPolicyRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Policy, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":setIamPolicy");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct TestIamPermissionsRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> TestIamPermissionsRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::TestIamPermissionsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.resource);
                        output.push_str(":testIamPermissions");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                pub mod messages {
                    pub mod params {
                        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                        pub enum GetView {}
                        impl GetView {
                            pub fn as_str(self) -> &'static str {
                                match self {}
                            }
                        }
                        impl ::std::fmt::Display for GetView {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.write_str(self.as_str())
                            }
                        }
                        impl ::serde::Serialize for GetView {
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                            where
                                S: ::serde::ser::Serializer,
                            {
                                serializer.serialize_str(self.as_str())
                            }
                        }
                        impl<'de> ::serde::Deserialize<'de> for GetView {
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: ::serde::de::Deserializer<'de>,
                            {
                                let value: &'de str = <&str>::deserialize(deserializer)?;
                                Ok(match value {
                                    _ => {
                                        return Err(::serde::de::Error::custom(format!(
                                            "invalid enum for #name: {}",
                                            value
                                        )))
                                    }
                                })
                            }
                        }
                    }
                    pub struct MessagesActions<'a, A> {
                        pub(super) reqwest: &'a reqwest::Client,
                        pub(super) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> MessagesActions<'a, A> {
                        #[doc = "Creates a message and sends a notification to the Cloud Pub/Sub topic. If\nconfigured, the MLLP adapter listens to messages created by this method and\nsends those back to the hospital. A successful response indicates the\nmessage has been persisted to storage and a Cloud Pub/Sub notification has\nbeen sent. Sending to the hospital by the MLLP adapter happens\nasynchronously."]
                        pub fn create(
                            &self,
                            request: crate::schemas::CreateMessageRequest,
                            parent: impl Into<String>,
                        ) -> CreateRequestBuilder<A> {
                            CreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Deletes an HL7v2 message."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets an HL7v2 message."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                view: None,
                            }
                        }
                        #[doc = "Ingests a new HL7v2 message from the hospital and sends a notification to\nthe Cloud Pub/Sub topic. Return is an HL7v2 ACK message if the message was\nsuccessfully stored. Otherwise an error is returned.  If an identical\nHL7v2 message is created twice only one resource is created on the server\nand no error is reported."]
                        pub fn ingest(
                            &self,
                            request: crate::schemas::IngestMessageRequest,
                            parent: impl Into<String>,
                        ) -> IngestRequestBuilder<A> {
                            IngestRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Lists all the messages in the given HL7v2 store with support for filtering.\n\nNote: HL7v2 messages are indexed asynchronously, so there might be a slight\ndelay between the time a message is created and when it can be found\nthrough a filter."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                filter: None,
                                order_by: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Update the message."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::Message,
                            name: impl Into<String>,
                        ) -> PatchRequestBuilder<A> {
                            PatchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                update_mask: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::CreateMessageRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Message, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/messages");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        name: String,
                        view: Option<crate::messages::params::GetView>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                        #[doc = "Specifies which parts of the Message resource should be returned\nin the response."]
                        pub fn view(
                            &mut self,
                            value: crate::messages::params::GetView,
                        ) -> &mut Self {
                            self.view = Some(value);
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Message, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("view", &self.view)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct IngestRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::IngestMessageRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> IngestRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<
                            crate::schemas::IngestMessageResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/messages:ingest");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        parent: String,
                        filter: Option<String>,
                        order_by: Option<String>,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "Restricts messages returned to those matching a filter. Syntax:\nhttps://cloud.google.com/appengine/docs/standard/python/search/query_strings\n\nFields/functions available for filtering are:\n\n*  `message_type`, from the MSH-9 segment; for example\n`NOT message_type = \"ADT\"`\n*  `send_date` or `sendDate`, the YYYY-MM-DD date the message was sent in\nthe dataset's time_zone, from the MSH-7 segment; for example\n`send_date < \"2017-01-02\"`\n*  `send_time`, the timestamp when the message was sent, using the\nRFC3339 time format for comparisons, from the MSH-7 segment; for example\n`send_time < \"2017-01-02T00:00:00-05:00\"`\n*  `send_facility`, the care center that the message came from, from the\nMSH-4 segment; for example `send_facility = \"ABC\"`\n*  `HL7RegExp(expr)`, which does regular expression matching of `expr`\nagainst the message payload using re2 (http://code.google.com/p/re2/)\nsyntax; for example `HL7RegExp(\"^.*\\|.*\\|EMERG\")`\n*  `PatientId(value, type)`, which matches if the message lists a patient\nhaving an ID of the given value and type in the PID-2, PID-3, or PID-4\nsegments; for example `PatientId(\"123456\", \"MRN\")`\n*  `labels.x`, a string value of the label with key `x` as set using the\nMessage.labels\nmap, for example `labels.\"priority\"=\"high\"`. The operator `:*` can be used\nto assert the existence of a label, for example `labels.\"priority\":*`.\n\nLimitations on conjunctions:\n\n*  Negation on the patient ID function or the labels field is not\nsupported, for example these queries are invalid:\n`NOT PatientId(\"123456\", \"MRN\")`, `NOT labels.\"tag1\":*`,\n`NOT labels.\"tag2\"=\"val2\"`.\n*  Conjunction of multiple patient ID functions is not supported, for\nexample this query is invalid:\n`PatientId(\"123456\", \"MRN\") AND PatientId(\"456789\", \"MRN\")`.\n*  Conjunction of multiple labels fields is also not supported, for\nexample this query is invalid: `labels.\"tag1\":* AND labels.\"tag2\"=\"val2\"`.\n*  Conjunction of one patient ID function, one labels field and conditions\non other fields is supported, for example this query is valid:\n`PatientId(\"123456\", \"MRN\") AND labels.\"tag1\":* AND message_type = \"ADT\"`."]
                        pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "Orders messages returned by the specified order_by clause.\nSyntax: https://cloud.google.com/apis/design/design_patterns#sorting_order\n\nFields available for ordering are:\n\n*  `send_time`"]
                        pub fn order_by(&mut self, value: impl Into<String>) -> &mut Self {
                            self.order_by = Some(value.into());
                            self
                        }
                        #[doc = "Limit on the number of messages to return in a single response.\nIf zero the default page size of 100 is used."]
                        pub fn page_size(&mut self, value: i32) -> &mut Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "The next_page_token value returned from the previous List request, if any."]
                        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn iter_messages<T>(
                            &'a mut self,
                        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                        {
                            struct ItemIter<'a, M, T> {
                                method: &'a mut M,
                                finished: bool,
                                items_iter: Option<::std::vec::IntoIter<T>>,
                            }
                            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                            where
                                M: crate::IterableMethod,
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                type Item = Result<T, Box<dyn ::std::error::Error>>;
                                fn next(
                                    &mut self,
                                ) -> Option<Result<T, Box<dyn ::std::error::Error>>>
                                {
                                    use ::field_selector::FieldSelector;
                                    #[derive(:: serde :: Deserialize, FieldSelector)]
                                    struct Resp<T>
                                    where
                                        T: FieldSelector,
                                    {
                                        #[serde(rename = "messages")]
                                        items: Option<Vec<T>>,
                                        #[serde(rename = "nextPageToken")]
                                        next_page_token: Option<String>,
                                    }
                                    loop {
                                        if let Some(iter) = self.items_iter.as_mut() {
                                            match iter.next() {
                                                Some(v) => return Some(Ok(v)),
                                                None => {}
                                            }
                                        }
                                        if self.finished {
                                            return None;
                                        }
                                        let resp: Resp<T> = match self.method.execute() {
                                            Ok(r) => r,
                                            Err(err) => return Some(Err(err)),
                                        };
                                        if let Some(next_page_token) = resp.next_page_token {
                                            self.method.set_page_token(next_page_token);
                                        } else {
                                            self.finished = true;
                                        }
                                        self.items_iter = resp.items.map(|i| i.into_iter());
                                    }
                                }
                            }
                            ItemIter {
                                method: self,
                                finished: false,
                                items_iter: None,
                            }
                        }
                        pub fn iter<T>(
                            &'a mut self,
                        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                        {
                            crate::PageIter {
                                method: self,
                                finished: false,
                                _phantom: ::std::default::Default::default(),
                            }
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<
                            crate::schemas::ListMessagesResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.parent);
                            output.push_str("/messages");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("filter", &self.filter)]);
                            let req = req.query(&[("orderBy", &self.order_by)]);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::Message,
                        name: String,
                        update_mask: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                        #[doc = "The update mask applies to the resource. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask\nOnly the `labels` field is allowed to be updated.\nThe labels in the request will be merged with the existing set of labels.\nExisting labels with the same keys will be updated."]
                        pub fn update_mask(&mut self, value: impl Into<String>) -> &mut Self {
                            self.update_mask = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "Data format for response."]
                        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                            self.alt = Some(value);
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "Selector specifying which fields to include in a partial response."]
                        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                            self.fields = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                            self.xgafv = Some(value);
                            self
                        }
                        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            self._execute()
                        }
                        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.text()?)
                        }
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Message, Box<dyn ::std::error::Error>>
                        {
                            self.execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            if self.fields.is_none() {
                                self.fields = Some(T::field_selector());
                            }
                            let req = self._request(&self._path());
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://healthcare.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            output.push_str(&self.name);
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            let req = req.query(&[("updateMask", &self.update_mask)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let req = req.bearer_auth(
                                auth.token::<_, &str>(&[
                                    "https://www.googleapis.com/auth/cloud-platform",
                                ])
                                .unwrap()
                                .access_token,
                            );
                            req
                        }
                    }
                }
            }
            pub mod operations {
                pub mod params {}
                pub struct OperationsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> OperationsActions<'a, A> {
                    #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists operations that match the specified filter in the request. If the\nserver doesn't support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id."]
                    pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "The standard list filter."]
                    pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "The standard list page size."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The standard list page token."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "Data format for response."]
                    pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                        self.alt = Some(value);
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "Selector specifying which fields to include in a partial response."]
                    pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fields = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                        self.xgafv = Some(value);
                        self
                    }
                    pub fn iter_operations<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "operations")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
                    }
                    pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                    #[doc = r" TODO: Remove once development debugging is no longer a priority."]
                    pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.text()?)
                    }
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListOperationsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        if self.fields.is_none() {
                            self.fields = Some(T::field_selector());
                        }
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://healthcare.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        output.push_str(&self.name);
                        output.push_str("/operations");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
        }
    }
}
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}
