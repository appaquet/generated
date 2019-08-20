pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LighthouseAuditResultV5 {
        #[doc = "The description of the audit."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Freeform details section of the audit."]
        #[serde(rename = "details", default)]
        pub details: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The value that should be displayed on the UI for this audit."]
        #[serde(rename = "displayValue", default)]
        pub display_value: Option<String>,
        #[doc = "An error message from a thrown error inside the audit."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[doc = "An explanation of the errors in the audit."]
        #[serde(rename = "explanation", default)]
        pub explanation: Option<String>,
        #[doc = "The audit's id."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[serde(rename = "score", default)]
        pub score: Option<::serde_json::Value>,
        #[doc = "The enumerated score display mode."]
        #[serde(rename = "scoreDisplayMode", default)]
        pub score_display_mode: Option<String>,
        #[doc = "The human readable title."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[serde(rename = "warnings", default)]
        pub warnings: Option<::serde_json::Value>,
    }
    impl ::field_selector::FieldSelector for LighthouseAuditResultV5 {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LighthouseCategoryV5AuditRefsItems {
        #[doc = "The category group that the audit belongs to (optional)."]
        #[serde(rename = "group", default)]
        pub group: Option<String>,
        #[doc = "The audit ref id."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The weight this audit's score has on the overall category score."]
        #[serde(rename = "weight", default)]
        pub weight: Option<f64>,
    }
    impl ::field_selector::FieldSelector for LighthouseCategoryV5AuditRefsItems {
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
    pub struct LighthouseCategoryV5 {
        #[doc = "An array of references to all the audit members of this category."]
        #[serde(rename = "auditRefs", default)]
        pub audit_refs: Option<Vec<crate::schemas::LighthouseCategoryV5AuditRefsItems>>,
        #[doc = "A more detailed description of the category and its importance."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The string identifier of the category."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "A description for the manual audits in the category."]
        #[serde(rename = "manualDescription", default)]
        pub manual_description: Option<String>,
        #[serde(rename = "score", default)]
        pub score: Option<::serde_json::Value>,
        #[doc = "The human-friendly name of the category."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseCategoryV5 {
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
    pub struct LighthouseResultV5Categories {
        #[doc = "The accessibility category, containing all accessibility related audits."]
        #[serde(rename = "accessibility", default)]
        pub accessibility: Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The best practices category, containing all web best practice related audits."]
        #[serde(rename = "best-practices", default)]
        pub best_practices: Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The performance category, containing all performance related audits."]
        #[serde(rename = "performance", default)]
        pub performance: Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The Progressive-Web-App (PWA) category, containing all pwa related audits."]
        #[serde(rename = "pwa", default)]
        pub pwa: Option<crate::schemas::LighthouseCategoryV5>,
        #[doc = "The Search-Engine-Optimization (SEO) category, containing all seo related audits."]
        #[serde(rename = "seo", default)]
        pub seo: Option<crate::schemas::LighthouseCategoryV5>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5Categories {
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
    pub struct LighthouseResultV5CategoryGroupsAdditionalProperties {
        #[doc = "An optional human readable description of the category group."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The title of the category group."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5CategoryGroupsAdditionalProperties {
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
    pub struct LighthouseResultV5ConfigSettings {
        #[doc = "The form factor the emulation should use."]
        #[serde(rename = "emulatedFormFactor", default)]
        pub emulated_form_factor: Option<String>,
        #[doc = "The locale setting."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[serde(rename = "onlyCategories", default)]
        pub only_categories: Option<::serde_json::Value>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5ConfigSettings {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LighthouseResultV5Environment {
        #[doc = "The benchmark index number that indicates rough device class."]
        #[serde(rename = "benchmarkIndex", default)]
        pub benchmark_index: Option<f64>,
        #[doc = "The user agent string of the version of Chrome used."]
        #[serde(rename = "hostUserAgent", default)]
        pub host_user_agent: Option<String>,
        #[doc = "The user agent string that was sent over the network."]
        #[serde(rename = "networkUserAgent", default)]
        pub network_user_agent: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5Environment {
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
    pub struct LighthouseResultV5I18NRendererFormattedStrings {
        #[doc = "The tooltip text on an expandable chevron icon."]
        #[serde(rename = "auditGroupExpandTooltip", default)]
        pub audit_group_expand_tooltip: Option<String>,
        #[doc = "The label for the initial request in a critical request chain."]
        #[serde(rename = "crcInitialNavigation", default)]
        pub crc_initial_navigation: Option<String>,
        #[doc = "The label for values shown in the summary of critical request chains."]
        #[serde(rename = "crcLongestDurationLabel", default)]
        pub crc_longest_duration_label: Option<String>,
        #[doc = "The label shown next to an audit or metric that has had an error."]
        #[serde(rename = "errorLabel", default)]
        pub error_label: Option<String>,
        #[doc = "The error string shown next to an erroring audit."]
        #[serde(rename = "errorMissingAuditInfo", default)]
        pub error_missing_audit_info: Option<String>,
        #[doc = "The title of the lab data performance category."]
        #[serde(rename = "labDataTitle", default)]
        pub lab_data_title: Option<String>,
        #[doc = "The disclaimer shown under performance explaning that the network can vary."]
        #[serde(rename = "lsPerformanceCategoryDescription", default)]
        pub ls_performance_category_description: Option<String>,
        #[doc = "The heading shown above a list of audits that were not computerd in the run."]
        #[serde(rename = "manualAuditsGroupTitle", default)]
        pub manual_audits_group_title: Option<String>,
        #[doc = "The heading shown above a list of audits that do not apply to a page."]
        #[serde(rename = "notApplicableAuditsGroupTitle", default)]
        pub not_applicable_audits_group_title: Option<String>,
        #[doc = "The heading for the estimated page load savings opportunity of an audit."]
        #[serde(rename = "opportunityResourceColumnLabel", default)]
        pub opportunity_resource_column_label: Option<String>,
        #[doc = "The heading for the estimated page load savings of opportunity audits."]
        #[serde(rename = "opportunitySavingsColumnLabel", default)]
        pub opportunity_savings_column_label: Option<String>,
        #[doc = "The heading that is shown above a list of audits that are passing."]
        #[serde(rename = "passedAuditsGroupTitle", default)]
        pub passed_audits_group_title: Option<String>,
        #[doc = "The label that explains the score gauges scale (0-49, 50-89, 90-100)."]
        #[serde(rename = "scorescaleLabel", default)]
        pub scorescale_label: Option<String>,
        #[doc = "The label shown preceding important warnings that may have invalidated an entire report."]
        #[serde(rename = "toplevelWarningsMessage", default)]
        pub toplevel_warnings_message: Option<String>,
        #[doc = "The disclaimer shown below a performance metric value."]
        #[serde(rename = "varianceDisclaimer", default)]
        pub variance_disclaimer: Option<String>,
        #[doc = "The label shown above a bulleted list of warnings."]
        #[serde(rename = "warningHeader", default)]
        pub warning_header: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5I18NRendererFormattedStrings {
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
    pub struct LighthouseResultV5I18N {
        #[doc = "Internationalized strings that are formatted to the locale in configSettings."]
        #[serde(rename = "rendererFormattedStrings", default)]
        pub renderer_formatted_strings:
            Option<crate::schemas::LighthouseResultV5I18NRendererFormattedStrings>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5I18N {
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
    pub struct LighthouseResultV5RuntimeError {
        #[doc = "The enumerated Lighthouse Error code."]
        #[serde(rename = "code", default)]
        pub code: Option<String>,
        #[doc = "A human readable message explaining the error code."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5RuntimeError {
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
    pub struct LighthouseResultV5StackPacksItems {
        #[doc = "The stack pack advice strings."]
        #[serde(rename = "descriptions", default)]
        pub descriptions: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The stack pack icon data uri."]
        #[serde(rename = "iconDataURL", default)]
        pub icon_data_url: Option<String>,
        #[doc = "The stack pack id."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The stack pack title."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5StackPacksItems {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LighthouseResultV5Timing {
        #[doc = "The total duration of Lighthouse's run."]
        #[serde(rename = "total", default)]
        pub total: Option<f64>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5Timing {
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
    pub struct LighthouseResultV5 {
        #[doc = "Map of audits in the LHR."]
        #[serde(rename = "audits", default)]
        pub audits:
            Option<::std::collections::BTreeMap<String, crate::schemas::LighthouseAuditResultV5>>,
        #[doc = "Map of categories in the LHR."]
        #[serde(rename = "categories", default)]
        pub categories: Option<crate::schemas::LighthouseResultV5Categories>,
        #[doc = "Map of category groups in the LHR."]
        #[serde(rename = "categoryGroups", default)]
        pub category_groups: Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::LighthouseResultV5CategoryGroupsAdditionalProperties,
            >,
        >,
        #[doc = "The configuration settings for this LHR."]
        #[serde(rename = "configSettings", default)]
        pub config_settings: Option<crate::schemas::LighthouseResultV5ConfigSettings>,
        #[doc = "Environment settings that were used when making this LHR."]
        #[serde(rename = "environment", default)]
        pub environment: Option<crate::schemas::LighthouseResultV5Environment>,
        #[doc = "The time that this run was fetched."]
        #[serde(rename = "fetchTime", default)]
        pub fetch_time: Option<String>,
        #[doc = "The final resolved url that was audited."]
        #[serde(rename = "finalUrl", default)]
        pub final_url: Option<String>,
        #[doc = "The internationalization strings that are required to render the LHR."]
        #[serde(rename = "i18n", default)]
        pub i_1_8n: Option<crate::schemas::LighthouseResultV5I18N>,
        #[doc = "The lighthouse version that was used to generate this LHR."]
        #[serde(rename = "lighthouseVersion", default)]
        pub lighthouse_version: Option<String>,
        #[doc = "The original requested url."]
        #[serde(rename = "requestedUrl", default)]
        pub requested_url: Option<String>,
        #[doc = "List of all run warnings in the LHR. Will always output to at least `[]`."]
        #[serde(rename = "runWarnings", default)]
        pub run_warnings: Option<Vec<::serde_json::Value>>,
        #[doc = "A top-level error message that, if present, indicates a serious enough problem that this Lighthouse result may need to be discarded."]
        #[serde(rename = "runtimeError", default)]
        pub runtime_error: Option<crate::schemas::LighthouseResultV5RuntimeError>,
        #[doc = "The Stack Pack advice strings."]
        #[serde(rename = "stackPacks", default)]
        pub stack_packs: Option<Vec<crate::schemas::LighthouseResultV5StackPacksItems>>,
        #[doc = "Timing information for this LHR."]
        #[serde(rename = "timing", default)]
        pub timing: Option<crate::schemas::LighthouseResultV5Timing>,
        #[doc = "The user agent that was used to run this LHR."]
        #[serde(rename = "userAgent", default)]
        pub user_agent: Option<String>,
    }
    impl ::field_selector::FieldSelector for LighthouseResultV5 {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PagespeedApiLoadingExperienceV5MetricsAdditionalPropertiesDistributionsItems {
        #[serde(rename = "max", default)]
        pub max: Option<i32>,
        #[serde(rename = "min", default)]
        pub min: Option<i32>,
        #[serde(rename = "proportion", default)]
        pub proportion: Option<f64>,
    }
    impl ::field_selector::FieldSelector
        for PagespeedApiLoadingExperienceV5MetricsAdditionalPropertiesDistributionsItems
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PagespeedApiLoadingExperienceV5MetricsAdditionalProperties { # [ serde ( rename = "category" , default ) ] pub category : Option < String > , # [ serde ( rename = "distributions" , default ) ] pub distributions : Option < Vec < crate :: schemas :: PagespeedApiLoadingExperienceV5MetricsAdditionalPropertiesDistributionsItems > > , # [ serde ( rename = "percentile" , default ) ] pub percentile : Option < i32 > , }
    impl ::field_selector::FieldSelector
        for PagespeedApiLoadingExperienceV5MetricsAdditionalProperties
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PagespeedApiLoadingExperienceV5 {
        #[doc = "The url, pattern or origin which the metrics are on."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[serde(rename = "initial_url", default)]
        pub initial_url: Option<String>,
        #[serde(rename = "metrics", default)]
        pub metrics: Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::PagespeedApiLoadingExperienceV5MetricsAdditionalProperties,
            >,
        >,
        #[serde(rename = "overall_category", default)]
        pub overall_category: Option<String>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiLoadingExperienceV5 {
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
    pub struct PagespeedApiPagespeedResponseV5Version {
        #[doc = "The major version number of PageSpeed used to generate these results."]
        #[serde(rename = "major", default)]
        pub major: Option<i32>,
        #[doc = "The minor version number of PageSpeed used to generate these results."]
        #[serde(rename = "minor", default)]
        pub minor: Option<i32>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV5Version {
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
    pub struct PagespeedApiPagespeedResponseV5 {
        #[doc = "The UTC timestamp of this analysis."]
        #[serde(rename = "analysisUTCTimestamp", default)]
        pub analysis_utc_timestamp: Option<String>,
        #[doc = "The captcha verify result"]
        #[serde(rename = "captchaResult", default)]
        pub captcha_result: Option<String>,
        #[doc = "Canonicalized and final URL for the document, after following page redirects (if any)."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of result."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Lighthouse response for the audit url as an object."]
        #[serde(rename = "lighthouseResult", default)]
        pub lighthouse_result: Option<crate::schemas::LighthouseResultV5>,
        #[doc = "Metrics of end users' page loading experience."]
        #[serde(rename = "loadingExperience", default)]
        pub loading_experience: Option<crate::schemas::PagespeedApiLoadingExperienceV5>,
        #[doc = "Metrics of the aggregated page loading experience of the origin"]
        #[serde(rename = "originLoadingExperience", default)]
        pub origin_loading_experience: Option<crate::schemas::PagespeedApiLoadingExperienceV5>,
        #[doc = "The version of PageSpeed used to generate these results."]
        #[serde(rename = "version", default)]
        pub version: Option<crate::schemas::PagespeedApiPagespeedResponseV5Version>,
    }
    impl ::field_selector::FieldSelector for PagespeedApiPagespeedResponseV5 {
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
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
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
    #[doc = "Actions that can be performed on the pagespeedapi resource"]
    pub fn pagespeedapi(&self) -> crate::pagespeedapi::PagespeedapiActions<A> {
        crate::pagespeedapi::PagespeedapiActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod pagespeedapi {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum RunpagespeedCategory {
            Accessibility,
            BestPractices,
            Performance,
            Pwa,
            Seo,
        }
        impl RunpagespeedCategory {
            pub fn as_str(self) -> &'static str {
                match self {
                    RunpagespeedCategory::Accessibility => "accessibility",
                    RunpagespeedCategory::BestPractices => "best-practices",
                    RunpagespeedCategory::Performance => "performance",
                    RunpagespeedCategory::Pwa => "pwa",
                    RunpagespeedCategory::Seo => "seo",
                }
            }
        }
        impl ::std::fmt::Display for RunpagespeedCategory {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RunpagespeedCategory {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RunpagespeedCategory {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "accessibility" => RunpagespeedCategory::Accessibility,
                    "best-practices" => RunpagespeedCategory::BestPractices,
                    "performance" => RunpagespeedCategory::Performance,
                    "pwa" => RunpagespeedCategory::Pwa,
                    "seo" => RunpagespeedCategory::Seo,
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
        pub enum RunpagespeedStrategy {
            #[doc = "Fetch and analyze the URL for desktop browsers"]
            Desktop,
            #[doc = "Fetch and analyze the URL for mobile devices"]
            Mobile,
        }
        impl RunpagespeedStrategy {
            pub fn as_str(self) -> &'static str {
                match self {
                    RunpagespeedStrategy::Desktop => "desktop",
                    RunpagespeedStrategy::Mobile => "mobile",
                }
            }
        }
        impl ::std::fmt::Display for RunpagespeedStrategy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for RunpagespeedStrategy {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for RunpagespeedStrategy {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "desktop" => RunpagespeedStrategy::Desktop,
                    "mobile" => RunpagespeedStrategy::Mobile,
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
    pub struct PagespeedapiActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PagespeedapiActions<'a, A> {
        #[doc = "Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information."]
        pub fn runpagespeed(&self, url: impl Into<String>) -> RunpagespeedRequestBuilder<A> {
            RunpagespeedRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                url: url.into(),
                category: None,
                locale: None,
                strategy: None,
                utm_campaign: None,
                utm_source: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct RunpagespeedRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        url: String,
        category: Option<crate::pagespeedapi::params::RunpagespeedCategory>,
        locale: Option<String>,
        strategy: Option<crate::pagespeedapi::params::RunpagespeedStrategy>,
        utm_campaign: Option<String>,
        utm_source: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RunpagespeedRequestBuilder<'a, A> {
        #[doc = "A Lighthouse category to run; if none are given, only Performance category will be run"]
        pub fn category(
            &mut self,
            value: crate::pagespeedapi::params::RunpagespeedCategory,
        ) -> &mut Self {
            self.category = Some(value);
            self
        }
        #[doc = "The locale used to localize formatted results"]
        pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
            self.locale = Some(value.into());
            self
        }
        #[doc = "The analysis strategy (desktop or mobile) to use, and desktop is the default"]
        pub fn strategy(
            &mut self,
            value: crate::pagespeedapi::params::RunpagespeedStrategy,
        ) -> &mut Self {
            self.strategy = Some(value);
            self
        }
        #[doc = "Campaign name for analytics."]
        pub fn utm_campaign(&mut self, value: impl Into<String>) -> &mut Self {
            self.utm_campaign = Some(value.into());
            self
        }
        #[doc = "Campaign source for analytics."]
        pub fn utm_source(&mut self, value: impl Into<String>) -> &mut Self {
            self.utm_source = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
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
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
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
        ) -> Result<crate::schemas::PagespeedApiPagespeedResponseV5, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/pagespeedonline/v5/".to_owned();
            output.push_str("runPagespeed");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("url", &self.url)]);
            let req = req.query(&[("category", &self.category)]);
            let req = req.query(&[("locale", &self.locale)]);
            let req = req.query(&[("strategy", &self.strategy)]);
            let req = req.query(&[("utm_campaign", &self.utm_campaign)]);
            let req = req.query(&[("utm_source", &self.utm_source)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            req
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
