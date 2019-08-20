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
    pub struct AlwaysOnVpnPackage {
        #[doc = "Disallows networking when the VPN is not connected."]
        #[serde(rename = "lockdownEnabled", default)]
        pub lockdown_enabled: Option<bool>,
        #[doc = "The package name of the VPN app."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for AlwaysOnVpnPackage {
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
    pub struct ApiLevelCondition {
        #[doc = "The minimum desired Android Framework API level. If the device doesn't meet the minimum requirement, this condition is satisfied. Must be greater than zero."]
        #[serde(rename = "minApiLevel", default)]
        pub min_api_level: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ApiLevelCondition {
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
    pub struct Application {
        #[doc = "The set of managed properties available to be pre-configured for the app."]
        #[serde(rename = "managedProperties", default)]
        pub managed_properties: Option<Vec<crate::schemas::ManagedProperty>>,
        #[doc = "The name of the app in the form enterprises/{enterpriseId}/applications/{package_name}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The permissions required by the app."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<crate::schemas::ApplicationPermission>>,
        #[doc = "The title of the app. Localized."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Application {
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
    pub enum ApplicationEventEventType {
        #[doc = "This value is disallowed."]
        ApplicationEventTypeUnspecified,
        #[doc = "The app was installed."]
        Installed,
        #[doc = "The app was changed, for example, a component was enabled or disabled."]
        Changed,
        #[doc = "The app data was cleared."]
        DataCleared,
        #[doc = "The app was removed."]
        Removed,
        #[doc = "A new version of the app has been installed, replacing the old version."]
        Replaced,
        #[doc = "The app was restarted."]
        Restarted,
        #[doc = "The app was pinned to the foreground."]
        Pinned,
        #[doc = "The app was unpinned."]
        Unpinned,
    }
    impl ApplicationEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationEventEventType::ApplicationEventTypeUnspecified => {
                    "APPLICATION_EVENT_TYPE_UNSPECIFIED"
                }
                ApplicationEventEventType::Installed => "INSTALLED",
                ApplicationEventEventType::Changed => "CHANGED",
                ApplicationEventEventType::DataCleared => "DATA_CLEARED",
                ApplicationEventEventType::Removed => "REMOVED",
                ApplicationEventEventType::Replaced => "REPLACED",
                ApplicationEventEventType::Restarted => "RESTARTED",
                ApplicationEventEventType::Pinned => "PINNED",
                ApplicationEventEventType::Unpinned => "UNPINNED",
            }
        }
    }
    impl ::std::fmt::Display for ApplicationEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationEventEventType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationEventEventType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_EVENT_TYPE_UNSPECIFIED" => {
                    ApplicationEventEventType::ApplicationEventTypeUnspecified
                }
                "INSTALLED" => ApplicationEventEventType::Installed,
                "CHANGED" => ApplicationEventEventType::Changed,
                "DATA_CLEARED" => ApplicationEventEventType::DataCleared,
                "REMOVED" => ApplicationEventEventType::Removed,
                "REPLACED" => ApplicationEventEventType::Replaced,
                "RESTARTED" => ApplicationEventEventType::Restarted,
                "PINNED" => ApplicationEventEventType::Pinned,
                "UNPINNED" => ApplicationEventEventType::Unpinned,
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
    pub struct ApplicationEvent {
        #[doc = "The creation time of the event."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "App event type."]
        #[serde(rename = "eventType", default)]
        pub event_type: Option<crate::schemas::ApplicationEventEventType>,
    }
    impl ::field_selector::FieldSelector for ApplicationEvent {
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
    pub struct ApplicationPermission {
        #[doc = "A longer description of the permission, providing more detail on what it affects. Localized."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The name of the permission. Localized."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "An opaque string uniquely identifying the permission. Not localized."]
        #[serde(rename = "permissionId", default)]
        pub permission_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ApplicationPermission {
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
    pub enum ApplicationPolicyDefaultPermissionPolicy {
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
        #[doc = "Automatically grant a permission."]
        Grant,
        #[doc = "Automatically deny a permission."]
        Deny,
    }
    impl ApplicationPolicyDefaultPermissionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationPolicyDefaultPermissionPolicy::PermissionPolicyUnspecified => {
                    "PERMISSION_POLICY_UNSPECIFIED"
                }
                ApplicationPolicyDefaultPermissionPolicy::Prompt => "PROMPT",
                ApplicationPolicyDefaultPermissionPolicy::Grant => "GRANT",
                ApplicationPolicyDefaultPermissionPolicy::Deny => "DENY",
            }
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyDefaultPermissionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyDefaultPermissionPolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyDefaultPermissionPolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    ApplicationPolicyDefaultPermissionPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => ApplicationPolicyDefaultPermissionPolicy::Prompt,
                "GRANT" => ApplicationPolicyDefaultPermissionPolicy::Grant,
                "DENY" => ApplicationPolicyDefaultPermissionPolicy::Deny,
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
    pub enum ApplicationPolicyDelegatedScopesItems {}
    impl ApplicationPolicyDelegatedScopesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyDelegatedScopesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyDelegatedScopesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyDelegatedScopesItems {
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationPolicyInstallType {
        #[doc = "Unspecified. Defaults to AVAILABLE."]
        InstallTypeUnspecified,
        #[doc = "The app is automatically installed and can be removed by the user."]
        Preinstalled,
        #[doc = "The app is automatically installed and can't be removed by the user."]
        ForceInstalled,
        #[doc = "The app is blocked and can't be installed. If the app was installed under a previous policy, it will be uninstalled."]
        Blocked,
        #[doc = "The app is available to install."]
        Available,
        #[doc = "The app is automatically installed and can't be removed by the user and will prevent setup from completion until installation is complete."]
        RequiredForSetup,
        #[doc = "The app is automatically installed in kiosk mode: it's set as the preferred home intent and whitelisted for lock task mode. Device setup won't complete until the app is installed. After installation, users won't be able to remove the app. You can only set this installType for one app per policy. When this is present in the policy, status bar will be automatically disabled."]
        Kiosk,
    }
    impl ApplicationPolicyInstallType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationPolicyInstallType::InstallTypeUnspecified => "INSTALL_TYPE_UNSPECIFIED",
                ApplicationPolicyInstallType::Preinstalled => "PREINSTALLED",
                ApplicationPolicyInstallType::ForceInstalled => "FORCE_INSTALLED",
                ApplicationPolicyInstallType::Blocked => "BLOCKED",
                ApplicationPolicyInstallType::Available => "AVAILABLE",
                ApplicationPolicyInstallType::RequiredForSetup => "REQUIRED_FOR_SETUP",
                ApplicationPolicyInstallType::Kiosk => "KIOSK",
            }
        }
    }
    impl ::std::fmt::Display for ApplicationPolicyInstallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationPolicyInstallType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationPolicyInstallType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INSTALL_TYPE_UNSPECIFIED" => ApplicationPolicyInstallType::InstallTypeUnspecified,
                "PREINSTALLED" => ApplicationPolicyInstallType::Preinstalled,
                "FORCE_INSTALLED" => ApplicationPolicyInstallType::ForceInstalled,
                "BLOCKED" => ApplicationPolicyInstallType::Blocked,
                "AVAILABLE" => ApplicationPolicyInstallType::Available,
                "REQUIRED_FOR_SETUP" => ApplicationPolicyInstallType::RequiredForSetup,
                "KIOSK" => ApplicationPolicyInstallType::Kiosk,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ApplicationPolicy {
        #[doc = "The default policy for all permissions requested by the app. If specified, this overrides the policy-level default_permission_policy which applies to all apps. It does not override the permission_grants which applies to all apps."]
        #[serde(rename = "defaultPermissionPolicy", default)]
        pub default_permission_policy:
            Option<crate::schemas::ApplicationPolicyDefaultPermissionPolicy>,
        #[doc = "The scopes delegated to the app from Android Device Policy."]
        #[serde(rename = "delegatedScopes", default)]
        pub delegated_scopes: Option<Vec<crate::schemas::ApplicationPolicyDelegatedScopesItems>>,
        #[doc = "Whether the app is disabled. When disabled, the app data is still preserved."]
        #[serde(rename = "disabled", default)]
        pub disabled: Option<bool>,
        #[doc = "The type of installation to perform."]
        #[serde(rename = "installType", default)]
        pub install_type: Option<crate::schemas::ApplicationPolicyInstallType>,
        #[doc = "Whether the app is allowed to lock itself in full-screen mode. DEPRECATED. Use InstallType KIOSK or kioskCustomLauncherEnabled to to configure a dedicated device."]
        #[serde(rename = "lockTaskAllowed", default)]
        pub lock_task_allowed: Option<bool>,
        #[doc = "Managed configuration applied to the app. The format for the configuration is dictated by the ManagedProperty values supported by the app. Each field name in the managed configuration must match the key field of the ManagedProperty. The field value must be compatible with the type of the ManagedProperty: <table> <tr><td><i>type</i></td><td><i>JSON value</i></td></tr> <tr><td>BOOL</td><td>true or false</td></tr> <tr><td>STRING</td><td>string</td></tr> <tr><td>INTEGER</td><td>number</td></tr> <tr><td>CHOICE</td><td>string</td></tr> <tr><td>MULTISELECT</td><td>array of strings</td></tr> <tr><td>HIDDEN</td><td>string</td></tr> <tr><td>BUNDLE_ARRAY</td><td>array of objects</td></tr> </table>"]
        #[serde(rename = "managedConfiguration", default)]
        pub managed_configuration:
            Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The managed configurations template for the app, saved from the managed configurations iframe. This field is ignored if managed_configuration is set."]
        #[serde(rename = "managedConfigurationTemplate", default)]
        pub managed_configuration_template: Option<crate::schemas::ManagedConfigurationTemplate>,
        #[doc = "The minimum version of the app that runs on the device. If set, the device attempts to update the app to at least this version code. If the app is not up-to-date, the device will contain a NonComplianceDetail with non_compliance_reason set to APP_NOT_UPDATED. The app must already be published to Google Play with a version code greater than or equal to this value. At most 20 apps may specify a minimum version code per policy."]
        #[serde(rename = "minimumVersionCode", default)]
        pub minimum_version_code: Option<i32>,
        #[doc = "The package name of the app. For example, com.google.android.youtube for the YouTube app."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
        #[doc = "Explicit permission grants or denials for the app. These values override the default_permission_policy and permission_grants which apply to all apps."]
        #[serde(rename = "permissionGrants", default)]
        pub permission_grants: Option<Vec<crate::schemas::PermissionGrant>>,
    }
    impl ::field_selector::FieldSelector for ApplicationPolicy {
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
    pub enum ApplicationReportApplicationSource {
        #[doc = "The app was sideloaded from an unspecified source."]
        ApplicationSourceUnspecified,
        #[doc = "This is a system app from the device's factory image."]
        SystemAppFactoryVersion,
        #[doc = "This is an updated system app."]
        SystemAppUpdatedVersion,
        #[doc = "The app was installed from the Google Play Store."]
        InstalledFromPlayStore,
    }
    impl ApplicationReportApplicationSource {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationReportApplicationSource::ApplicationSourceUnspecified => {
                    "APPLICATION_SOURCE_UNSPECIFIED"
                }
                ApplicationReportApplicationSource::SystemAppFactoryVersion => {
                    "SYSTEM_APP_FACTORY_VERSION"
                }
                ApplicationReportApplicationSource::SystemAppUpdatedVersion => {
                    "SYSTEM_APP_UPDATED_VERSION"
                }
                ApplicationReportApplicationSource::InstalledFromPlayStore => {
                    "INSTALLED_FROM_PLAY_STORE"
                }
            }
        }
    }
    impl ::std::fmt::Display for ApplicationReportApplicationSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationReportApplicationSource {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationReportApplicationSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION_SOURCE_UNSPECIFIED" => {
                    ApplicationReportApplicationSource::ApplicationSourceUnspecified
                }
                "SYSTEM_APP_FACTORY_VERSION" => {
                    ApplicationReportApplicationSource::SystemAppFactoryVersion
                }
                "SYSTEM_APP_UPDATED_VERSION" => {
                    ApplicationReportApplicationSource::SystemAppUpdatedVersion
                }
                "INSTALLED_FROM_PLAY_STORE" => {
                    ApplicationReportApplicationSource::InstalledFromPlayStore
                }
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
    pub enum ApplicationReportState {
        #[doc = "App is installed on the device"]
        Installed,
        #[doc = "App was removed from the device"]
        Removed,
    }
    impl ApplicationReportState {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationReportState::Installed => "INSTALLED",
                ApplicationReportState::Removed => "REMOVED",
            }
        }
    }
    impl ::std::fmt::Display for ApplicationReportState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationReportState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationReportState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INSTALLED" => ApplicationReportState::Installed,
                "REMOVED" => ApplicationReportState::Removed,
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
    pub struct ApplicationReport {
        #[doc = "The source of the package."]
        #[serde(rename = "applicationSource", default)]
        pub application_source: Option<crate::schemas::ApplicationReportApplicationSource>,
        #[doc = "The display name of the app."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "List of app events. The most recent 20 events are stored in the list."]
        #[serde(rename = "events", default)]
        pub events: Option<Vec<crate::schemas::ApplicationEvent>>,
        #[doc = "The package name of the app that installed this app."]
        #[serde(rename = "installerPackageName", default)]
        pub installer_package_name: Option<String>,
        #[doc = "List of keyed app states reported by the app."]
        #[serde(rename = "keyedAppStates", default)]
        pub keyed_app_states: Option<Vec<crate::schemas::KeyedAppState>>,
        #[doc = "Package name of the app."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
        #[doc = "The SHA-256 hash of the app's APK file, which can be used to verify the app hasn't been modified. Each byte of the hash value is represented as a two-digit hexadecimal number."]
        #[serde(rename = "packageSha256Hash", default)]
        pub package_sha_256_hash: Option<String>,
        #[doc = "The SHA-1 hash of each android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the app package. Each byte of each hash value is represented as a two-digit hexadecimal number."]
        #[serde(rename = "signingKeyCertFingerprints", default)]
        pub signing_key_cert_fingerprints: Option<Vec<String>>,
        #[doc = "Application state."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::ApplicationReportState>,
        #[doc = "The app version code, which can be used to determine whether one version is more recent than another."]
        #[serde(rename = "versionCode", default)]
        pub version_code: Option<i32>,
        #[doc = "The app version as displayed to the user."]
        #[serde(rename = "versionName", default)]
        pub version_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ApplicationReport {
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
    pub struct ApplicationReportingSettings {
        #[doc = "Whether removed apps are included in application reports."]
        #[serde(rename = "includeRemovedApps", default)]
        pub include_removed_apps: Option<bool>,
    }
    impl ::field_selector::FieldSelector for ApplicationReportingSettings {
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
    pub struct BlockAction {
        #[doc = "Number of days the policy is non-compliant before the device or work profile is blocked. To block access immediately, set to 0. blockAfterDays must be less than wipeAfterDays."]
        #[serde(rename = "blockAfterDays", default)]
        pub block_after_days: Option<i32>,
    }
    impl ::field_selector::FieldSelector for BlockAction {
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
    pub struct ChoosePrivateKeyRule {
        #[doc = "The package names for which outgoing requests are subject to this rule. If no package names are specified, then the rule applies to all packages. For each package name listed, the rule applies to that package and all other packages that shared the same Android UID. The SHA256 hash of the signing key signatures of each package_name will be verified against those provided by Play"]
        #[serde(rename = "packageNames", default)]
        pub package_names: Option<Vec<String>>,
        #[doc = "The alias of the private key to be used."]
        #[serde(rename = "privateKeyAlias", default)]
        pub private_key_alias: Option<String>,
        #[doc = "The URL pattern to match against the URL of the outgoing request. The pattern may contain asterisk (*) wildcards. Any URL is matched if unspecified."]
        #[serde(rename = "urlPattern", default)]
        pub url_pattern: Option<String>,
    }
    impl ::field_selector::FieldSelector for ChoosePrivateKeyRule {
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
    pub enum CommandErrorCode {
        #[doc = "There was no error."]
        CommandErrorCodeUnspecified,
        #[doc = "An unknown error occurred."]
        Unknown,
        #[doc = "The API level of the device does not support this command."]
        ApiLevel,
        #[doc = "The management mode (profile owner, device owner, etc.) does not support the command."]
        ManagementMode,
        #[doc = "The command has an invalid parameter value."]
        InvalidValue,
        #[doc = "The device doesn't support the command. Updating Android Device Policy to the latest version may resolve the issue."]
        Unsupported,
    }
    impl CommandErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                CommandErrorCode::CommandErrorCodeUnspecified => "COMMAND_ERROR_CODE_UNSPECIFIED",
                CommandErrorCode::Unknown => "UNKNOWN",
                CommandErrorCode::ApiLevel => "API_LEVEL",
                CommandErrorCode::ManagementMode => "MANAGEMENT_MODE",
                CommandErrorCode::InvalidValue => "INVALID_VALUE",
                CommandErrorCode::Unsupported => "UNSUPPORTED",
            }
        }
    }
    impl ::std::fmt::Display for CommandErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommandErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommandErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMAND_ERROR_CODE_UNSPECIFIED" => CommandErrorCode::CommandErrorCodeUnspecified,
                "UNKNOWN" => CommandErrorCode::Unknown,
                "API_LEVEL" => CommandErrorCode::ApiLevel,
                "MANAGEMENT_MODE" => CommandErrorCode::ManagementMode,
                "INVALID_VALUE" => CommandErrorCode::InvalidValue,
                "UNSUPPORTED" => CommandErrorCode::Unsupported,
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
    pub enum CommandType {
        #[doc = "This value is disallowed."]
        CommandTypeUnspecified,
        #[doc = "Lock the device, as if the lock screen timeout had expired."]
        Lock,
        #[doc = "Reset the user's password."]
        ResetPassword,
        #[doc = "Reboot the device. Only supported on API level 24+."]
        Reboot,
    }
    impl CommandType {
        pub fn as_str(self) -> &'static str {
            match self {
                CommandType::CommandTypeUnspecified => "COMMAND_TYPE_UNSPECIFIED",
                CommandType::Lock => "LOCK",
                CommandType::ResetPassword => "RESET_PASSWORD",
                CommandType::Reboot => "REBOOT",
            }
        }
    }
    impl ::std::fmt::Display for CommandType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommandType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommandType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMAND_TYPE_UNSPECIFIED" => CommandType::CommandTypeUnspecified,
                "LOCK" => CommandType::Lock,
                "RESET_PASSWORD" => CommandType::ResetPassword,
                "REBOOT" => CommandType::Reboot,
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
    pub enum CommandResetPasswordFlagsItems {}
    impl CommandResetPasswordFlagsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for CommandResetPasswordFlagsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommandResetPasswordFlagsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommandResetPasswordFlagsItems {
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
    pub struct Command {
        #[doc = "The timestamp at which the command was created. The timestamp is automatically generated by the server."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "The duration for which the command is valid. The command will expire if not executed by the device during this time. The default duration if unspecified is ten minutes. There is no maximum duration."]
        #[serde(rename = "duration", default)]
        pub duration: Option<String>,
        #[doc = "If the command failed, an error code explaining the failure. This is not set when the command is cancelled by the caller."]
        #[serde(rename = "errorCode", default)]
        pub error_code: Option<crate::schemas::CommandErrorCode>,
        #[doc = "For commands of type RESET_PASSWORD, optionally specifies the new password."]
        #[serde(rename = "newPassword", default)]
        pub new_password: Option<String>,
        #[doc = "The type of the command."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::CommandType>,
        #[doc = "For commands of type RESET_PASSWORD, optionally specifies flags."]
        #[serde(rename = "resetPasswordFlags", default)]
        pub reset_password_flags: Option<Vec<crate::schemas::CommandResetPasswordFlagsItems>>,
        #[doc = "The resource name of the user that owns the device in the form enterprises/{enterpriseId}/users/{userId}. This is automatically generated by the server based on the device the command is sent to."]
        #[serde(rename = "userName", default)]
        pub user_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Command {
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
    pub struct ComplianceRule {
        #[doc = "A condition which is satisfied if the Android Framework API level on the device doesn't meet a minimum requirement."]
        #[serde(rename = "apiLevelCondition", default)]
        pub api_level_condition: Option<crate::schemas::ApiLevelCondition>,
        #[doc = "If set to true, the rule includes a mitigating action to disable apps so that the device is effectively disabled, but app data is preserved. If the device is running an app in locked task mode, the app will be closed and a UI showing the reason for non-compliance will be displayed."]
        #[serde(rename = "disableApps", default)]
        pub disable_apps: Option<bool>,
        #[doc = "A condition which is satisfied if there exists any matching NonComplianceDetail for the device."]
        #[serde(rename = "nonComplianceDetailCondition", default)]
        pub non_compliance_detail_condition: Option<crate::schemas::NonComplianceDetailCondition>,
        #[doc = "If set, the rule includes a mitigating action to disable apps specified in the list, but app data is preserved."]
        #[serde(rename = "packageNamesToDisable", default)]
        pub package_names_to_disable: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ComplianceRule {
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
    pub enum DeviceAppliedState {
        #[doc = "This value is disallowed."]
        DeviceStateUnspecified,
        #[doc = "The device is active."]
        Active,
        #[doc = "The device is disabled."]
        Disabled,
        #[doc = "The device was deleted. This state will never be returned by an API call, but is used in the final status report published to Cloud Pub/Sub when the device acknowledges the deletion."]
        Deleted,
        #[doc = "The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied."]
        Provisioning,
    }
    impl DeviceAppliedState {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceAppliedState::DeviceStateUnspecified => "DEVICE_STATE_UNSPECIFIED",
                DeviceAppliedState::Active => "ACTIVE",
                DeviceAppliedState::Disabled => "DISABLED",
                DeviceAppliedState::Deleted => "DELETED",
                DeviceAppliedState::Provisioning => "PROVISIONING",
            }
        }
    }
    impl ::std::fmt::Display for DeviceAppliedState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceAppliedState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceAppliedState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_STATE_UNSPECIFIED" => DeviceAppliedState::DeviceStateUnspecified,
                "ACTIVE" => DeviceAppliedState::Active,
                "DISABLED" => DeviceAppliedState::Disabled,
                "DELETED" => DeviceAppliedState::Deleted,
                "PROVISIONING" => DeviceAppliedState::Provisioning,
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
    pub enum DeviceManagementMode {
        #[doc = "This value is disallowed."]
        ManagementModeUnspecified,
        #[doc = "Device owner. Android Device Policy has full control over the device."]
        DeviceOwner,
        #[doc = "Profile owner. Android Device Policy has control over a managed profile on the device."]
        ProfileOwner,
    }
    impl DeviceManagementMode {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceManagementMode::ManagementModeUnspecified => "MANAGEMENT_MODE_UNSPECIFIED",
                DeviceManagementMode::DeviceOwner => "DEVICE_OWNER",
                DeviceManagementMode::ProfileOwner => "PROFILE_OWNER",
            }
        }
    }
    impl ::std::fmt::Display for DeviceManagementMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceManagementMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceManagementMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MANAGEMENT_MODE_UNSPECIFIED" => DeviceManagementMode::ManagementModeUnspecified,
                "DEVICE_OWNER" => DeviceManagementMode::DeviceOwner,
                "PROFILE_OWNER" => DeviceManagementMode::ProfileOwner,
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
    pub enum DeviceState {
        #[doc = "This value is disallowed."]
        DeviceStateUnspecified,
        #[doc = "The device is active."]
        Active,
        #[doc = "The device is disabled."]
        Disabled,
        #[doc = "The device was deleted. This state will never be returned by an API call, but is used in the final status report published to Cloud Pub/Sub when the device acknowledges the deletion."]
        Deleted,
        #[doc = "The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied."]
        Provisioning,
    }
    impl DeviceState {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceState::DeviceStateUnspecified => "DEVICE_STATE_UNSPECIFIED",
                DeviceState::Active => "ACTIVE",
                DeviceState::Disabled => "DISABLED",
                DeviceState::Deleted => "DELETED",
                DeviceState::Provisioning => "PROVISIONING",
            }
        }
    }
    impl ::std::fmt::Display for DeviceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_STATE_UNSPECIFIED" => DeviceState::DeviceStateUnspecified,
                "ACTIVE" => DeviceState::Active,
                "DISABLED" => DeviceState::Disabled,
                "DELETED" => DeviceState::Deleted,
                "PROVISIONING" => DeviceState::Provisioning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Device {
        #[doc = "The API level of the Android platform version running on the device."]
        #[serde(rename = "apiLevel", default)]
        pub api_level: Option<i32>,
        #[doc = "Reports for apps installed on the device. This information is only available when application_reports_enabled is true in the device's policy."]
        #[serde(rename = "applicationReports", default)]
        pub application_reports: Option<Vec<crate::schemas::ApplicationReport>>,
        #[doc = "The name of the policy currently applied to the device."]
        #[serde(rename = "appliedPolicyName", default)]
        pub applied_policy_name: Option<String>,
        #[doc = "The version of the policy currently applied to the device."]
        #[serde(rename = "appliedPolicyVersion", default)]
        #[serde(with = "crate::parsed_string")]
        pub applied_policy_version: Option<i64>,
        #[doc = "The state currently applied to the device."]
        #[serde(rename = "appliedState", default)]
        pub applied_state: Option<crate::schemas::DeviceAppliedState>,
        #[doc = "Device settings information. This information is only available if deviceSettingsEnabled is true in the device's policy."]
        #[serde(rename = "deviceSettings", default)]
        pub device_settings: Option<crate::schemas::DeviceSettings>,
        #[doc = "If the device state is DISABLED, an optional message that is displayed on the device indicating the reason the device is disabled. This field can be modified by a patch request."]
        #[serde(rename = "disabledReason", default)]
        pub disabled_reason: Option<crate::schemas::UserFacingMessage>,
        #[doc = "Detailed information about displays on the device. This information is only available if displayInfoEnabled is true in the device's policy."]
        #[serde(rename = "displays", default)]
        pub displays: Option<Vec<crate::schemas::Display>>,
        #[doc = "The time of device enrollment."]
        #[serde(rename = "enrollmentTime", default)]
        pub enrollment_time: Option<String>,
        #[doc = "If the device was enrolled with an enrollment token with additional data provided, this field contains that data."]
        #[serde(rename = "enrollmentTokenData", default)]
        pub enrollment_token_data: Option<String>,
        #[doc = "If the device was enrolled with an enrollment token, this field contains the name of the token."]
        #[serde(rename = "enrollmentTokenName", default)]
        pub enrollment_token_name: Option<String>,
        #[doc = "Detailed information about the device hardware."]
        #[serde(rename = "hardwareInfo", default)]
        pub hardware_info: Option<crate::schemas::HardwareInfo>,
        #[doc = "Hardware status samples in chronological order. This information is only available if hardwareStatusEnabled is true in the device's policy."]
        #[serde(rename = "hardwareStatusSamples", default)]
        pub hardware_status_samples: Option<Vec<crate::schemas::HardwareStatus>>,
        #[doc = "Deprecated."]
        #[serde(rename = "lastPolicyComplianceReportTime", default)]
        pub last_policy_compliance_report_time: Option<String>,
        #[doc = "The last time the device fetched its policy."]
        #[serde(rename = "lastPolicySyncTime", default)]
        pub last_policy_sync_time: Option<String>,
        #[doc = "The last time the device sent a status report."]
        #[serde(rename = "lastStatusReportTime", default)]
        pub last_status_report_time: Option<String>,
        #[doc = "The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported."]
        #[serde(rename = "managementMode", default)]
        pub management_mode: Option<crate::schemas::DeviceManagementMode>,
        #[doc = "Events related to memory and storage measurements in chronological order. This information is only available if memoryInfoEnabled is true in the device's policy."]
        #[serde(rename = "memoryEvents", default)]
        pub memory_events: Option<Vec<crate::schemas::MemoryEvent>>,
        #[doc = "Memory information. This information is only available if memoryInfoEnabled is true in the device's policy."]
        #[serde(rename = "memoryInfo", default)]
        pub memory_info: Option<crate::schemas::MemoryInfo>,
        #[doc = "The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Device network information. This information is only available if networkInfoEnabled is true in the device's policy."]
        #[serde(rename = "networkInfo", default)]
        pub network_info: Option<crate::schemas::NetworkInfo>,
        #[doc = "Details about policy settings that the device is not compliant with."]
        #[serde(rename = "nonComplianceDetails", default)]
        pub non_compliance_details: Option<Vec<crate::schemas::NonComplianceDetail>>,
        #[doc = "Whether the device is compliant with its policy."]
        #[serde(rename = "policyCompliant", default)]
        pub policy_compliant: Option<bool>,
        #[doc = "The name of the policy applied to the device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device's user is applied. This field can be modified by a patch request. You can specify only the policyId when calling enterprises.devices.patch, as long as the policyId doesn\u{2019}t contain any slashes. The rest of the policy name is inferred."]
        #[serde(rename = "policyName", default)]
        pub policy_name: Option<String>,
        #[doc = "Power management events on the device in chronological order. This information is only available if powerManagementEventsEnabled is true in the device's policy."]
        #[serde(rename = "powerManagementEvents", default)]
        pub power_management_events: Option<Vec<crate::schemas::PowerManagementEvent>>,
        #[doc = "If the same physical device has been enrolled multiple times, this field contains its previous device names. The serial number is used as the unique identifier to determine if the same physical device has enrolled previously. The names are in chronological order."]
        #[serde(rename = "previousDeviceNames", default)]
        pub previous_device_names: Option<Vec<String>>,
        #[doc = "Device's security posture value that reflects how secure the device is."]
        #[serde(rename = "securityPosture", default)]
        pub security_posture: Option<crate::schemas::SecurityPosture>,
        #[doc = "Detailed information about the device software. This information is only available if softwareInfoEnabled is true in the device's policy."]
        #[serde(rename = "softwareInfo", default)]
        pub software_info: Option<crate::schemas::SoftwareInfo>,
        #[doc = "The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::DeviceState>,
        #[doc = "Map of selected system properties name and value related to the device."]
        #[serde(rename = "systemProperties", default)]
        pub system_properties: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The user who owns the device."]
        #[serde(rename = "user", default)]
        pub user: Option<crate::schemas::User>,
        #[doc = "The resource name of the user that owns this device in the form enterprises/{enterpriseId}/users/{userId}."]
        #[serde(rename = "userName", default)]
        pub user_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Device {
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
    pub enum DeviceSettingsEncryptionStatus {
        #[doc = "Unspecified. No device should have this type."]
        EncryptionStatusUnspecified,
        #[doc = "Encryption is not supported by the device."]
        Unsupported,
        #[doc = "Encryption is supported by the device, but is not currently active."]
        Inactive,
        #[doc = "Encryption is not currently active, but is currently being activated."]
        Activating,
        #[doc = "Encryption is active."]
        Active,
        #[doc = "Encryption is active, but an encryption key is not set by the user."]
        ActiveDefaultKey,
        #[doc = "Encryption is active, and the encryption key is tied to the user profile."]
        ActivePerUser,
    }
    impl DeviceSettingsEncryptionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceSettingsEncryptionStatus::EncryptionStatusUnspecified => {
                    "ENCRYPTION_STATUS_UNSPECIFIED"
                }
                DeviceSettingsEncryptionStatus::Unsupported => "UNSUPPORTED",
                DeviceSettingsEncryptionStatus::Inactive => "INACTIVE",
                DeviceSettingsEncryptionStatus::Activating => "ACTIVATING",
                DeviceSettingsEncryptionStatus::Active => "ACTIVE",
                DeviceSettingsEncryptionStatus::ActiveDefaultKey => "ACTIVE_DEFAULT_KEY",
                DeviceSettingsEncryptionStatus::ActivePerUser => "ACTIVE_PER_USER",
            }
        }
    }
    impl ::std::fmt::Display for DeviceSettingsEncryptionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceSettingsEncryptionStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceSettingsEncryptionStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENCRYPTION_STATUS_UNSPECIFIED" => {
                    DeviceSettingsEncryptionStatus::EncryptionStatusUnspecified
                }
                "UNSUPPORTED" => DeviceSettingsEncryptionStatus::Unsupported,
                "INACTIVE" => DeviceSettingsEncryptionStatus::Inactive,
                "ACTIVATING" => DeviceSettingsEncryptionStatus::Activating,
                "ACTIVE" => DeviceSettingsEncryptionStatus::Active,
                "ACTIVE_DEFAULT_KEY" => DeviceSettingsEncryptionStatus::ActiveDefaultKey,
                "ACTIVE_PER_USER" => DeviceSettingsEncryptionStatus::ActivePerUser,
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
    pub struct DeviceSettings {
        #[doc = "Whether ADB (https://developer.android.com/studio/command-line/adb.html) is enabled on the device."]
        #[serde(rename = "adbEnabled", default)]
        pub adb_enabled: Option<bool>,
        #[doc = "Whether developer mode is enabled on the device."]
        #[serde(rename = "developmentSettingsEnabled", default)]
        pub development_settings_enabled: Option<bool>,
        #[doc = "Encryption status from DevicePolicyManager."]
        #[serde(rename = "encryptionStatus", default)]
        pub encryption_status: Option<crate::schemas::DeviceSettingsEncryptionStatus>,
        #[doc = "Whether the device is secured with PIN/password."]
        #[serde(rename = "isDeviceSecure", default)]
        pub is_device_secure: Option<bool>,
        #[doc = "Whether the storage encryption is enabled."]
        #[serde(rename = "isEncrypted", default)]
        pub is_encrypted: Option<bool>,
        #[doc = "Whether installing apps from unknown sources is enabled."]
        #[serde(rename = "unknownSourcesEnabled", default)]
        pub unknown_sources_enabled: Option<bool>,
        #[doc = "Whether Verify Apps (Google Play Protect (https://support.google.com/googleplay/answer/2812853)) is enabled on the device."]
        #[serde(rename = "verifyAppsEnabled", default)]
        pub verify_apps_enabled: Option<bool>,
    }
    impl ::field_selector::FieldSelector for DeviceSettings {
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
    pub enum DisplayState {
        #[doc = "This value is disallowed."]
        DisplayStateUnspecified,
        #[doc = "Display is off."]
        Off,
        #[doc = "Display is on."]
        On,
        #[doc = "Display is dozing in a low power state"]
        Doze,
        #[doc = "Display is dozing in a suspended low power state."]
        Suspended,
    }
    impl DisplayState {
        pub fn as_str(self) -> &'static str {
            match self {
                DisplayState::DisplayStateUnspecified => "DISPLAY_STATE_UNSPECIFIED",
                DisplayState::Off => "OFF",
                DisplayState::On => "ON",
                DisplayState::Doze => "DOZE",
                DisplayState::Suspended => "SUSPENDED",
            }
        }
    }
    impl ::std::fmt::Display for DisplayState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DisplayState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DisplayState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISPLAY_STATE_UNSPECIFIED" => DisplayState::DisplayStateUnspecified,
                "OFF" => DisplayState::Off,
                "ON" => DisplayState::On,
                "DOZE" => DisplayState::Doze,
                "SUSPENDED" => DisplayState::Suspended,
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
    pub struct Display {
        #[doc = "Display density expressed as dots-per-inch."]
        #[serde(rename = "density", default)]
        pub density: Option<i32>,
        #[doc = "Unique display id."]
        #[serde(rename = "displayId", default)]
        pub display_id: Option<i32>,
        #[doc = "Display height in pixels."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "Name of the display."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Refresh rate of the display in frames per second."]
        #[serde(rename = "refreshRate", default)]
        pub refresh_rate: Option<i32>,
        #[doc = "State of the display."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::DisplayState>,
        #[doc = "Display width in pixels."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Display {
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
    pub struct EnrollmentToken {
        #[doc = "Optional, arbitrary data associated with the enrollment token. This could contain, for example, the ID of an org unit the device is assigned to after enrollment. After a device enrolls with the token, this data will be exposed in the enrollment_token_data field of the Device resource. The data must be 1024 characters or less; otherwise, the creation request will fail."]
        #[serde(rename = "additionalData", default)]
        pub additional_data: Option<String>,
        #[doc = "The length of time the enrollment token is valid, ranging from 1 minute to 30 days. If not specified, the default duration is 1 hour."]
        #[serde(rename = "duration", default)]
        pub duration: Option<String>,
        #[doc = "The expiration time of the token. This is a read-only field generated by the server."]
        #[serde(rename = "expirationTimestamp", default)]
        pub expiration_timestamp: Option<String>,
        #[doc = "The name of the enrollment token, which is generated by the server during creation, in the form enterprises/{enterpriseId}/enrollmentTokens/{enrollmentTokenId}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Whether the enrollment token is for one time use only. If the flag is set to true, only one device can use it for registration."]
        #[serde(rename = "oneTimeOnly", default)]
        pub one_time_only: Option<bool>,
        #[doc = "The name of the policy initially applied to the enrolled device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device\u{2019}s user is applied. If user_name is also not specified, enterprises/{enterpriseId}/policies/default is applied by default. When updating this field, you can specify only the policyId as long as the policyId doesn\u{2019}t contain any slashes. The rest of the policy name will be inferred."]
        #[serde(rename = "policyName", default)]
        pub policy_name: Option<String>,
        #[doc = "A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON."]
        #[serde(rename = "qrCode", default)]
        pub qr_code: Option<String>,
        #[doc = "The user associated with this enrollment token. If it's specified when the enrollment token is created and the user does not exist, the user will be created. This field must not contain personally identifiable information. Only the account_identifier field needs to be set."]
        #[serde(rename = "user", default)]
        pub user: Option<crate::schemas::User>,
        #[doc = "The token value that's passed to the device and authorizes the device to enroll. This is a read-only field generated by the server."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for EnrollmentToken {
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
    pub enum EnterpriseEnabledNotificationTypesItems {}
    impl EnterpriseEnabledNotificationTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for EnterpriseEnabledNotificationTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnterpriseEnabledNotificationTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnterpriseEnabledNotificationTypesItems {
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
    pub struct Enterprise {
        #[doc = "Deprecated and unused."]
        #[serde(rename = "appAutoApprovalEnabled", default)]
        pub app_auto_approval_enabled: Option<bool>,
        #[doc = "The types of Google Pub/Sub notifications enabled for the enterprise."]
        #[serde(rename = "enabledNotificationTypes", default)]
        pub enabled_notification_types:
            Option<Vec<crate::schemas::EnterpriseEnabledNotificationTypesItems>>,
        #[doc = "The name of the enterprise displayed to users."]
        #[serde(rename = "enterpriseDisplayName", default)]
        pub enterprise_display_name: Option<String>,
        #[doc = "An image displayed as a logo during device provisioning. Supported types are: image/bmp, image/gif, image/x-ico, image/jpeg, image/png, image/webp, image/vnd.wap.wbmp, image/x-adobe-dng."]
        #[serde(rename = "logo", default)]
        pub logo: Option<crate::schemas::ExternalData>,
        #[doc = "The name of the enterprise which is generated by the server during creation, in the form enterprises/{enterpriseId}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "A color in RGB format that indicates the predominant color to display in the device management app UI. The color components are stored as follows: (red << 16) | (green << 8) | blue, where the value of each component is between 0 and 255, inclusive."]
        #[serde(rename = "primaryColor", default)]
        pub primary_color: Option<i32>,
        #[doc = "The topic that Cloud Pub/Sub notifications are published to, in the form projects/{project}/topics/{topic}. This field is only required if Pub/Sub notifications are enabled."]
        #[serde(rename = "pubsubTopic", default)]
        pub pubsub_topic: Option<String>,
        #[doc = "Sign-in details of the enterprise. Maximum of 1 SigninDetail is supported."]
        #[serde(rename = "signinDetails", default)]
        pub signin_details: Option<Vec<crate::schemas::SigninDetail>>,
        #[doc = "Terms and conditions that must be accepted when provisioning a device for this enterprise. A page of terms is generated for each value in this list."]
        #[serde(rename = "termsAndConditions", default)]
        pub terms_and_conditions: Option<Vec<crate::schemas::TermsAndConditions>>,
    }
    impl ::field_selector::FieldSelector for Enterprise {
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
    pub struct ExternalData {
        #[doc = "The base-64 encoded SHA-256 hash of the content hosted at url. If the content doesn't match this hash, Android Device Policy won't use the data."]
        #[serde(rename = "sha256Hash", default)]
        pub sha_256_hash: Option<String>,
        #[doc = "The absolute URL to the data, which must use either the http or https scheme. Android Device Policy doesn't provide any credentials in the GET request, so the URL must be publicly accessible. Including a long, random component in the URL may be used to prevent attackers from discovering the URL."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for ExternalData {
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
    pub struct HardwareInfo {
        #[doc = "Battery shutdown temperature thresholds in Celsius for each battery on the device."]
        #[serde(rename = "batteryShutdownTemperatures", default)]
        pub battery_shutdown_temperatures: Option<Vec<f32>>,
        #[doc = "Battery throttling temperature thresholds in Celsius for each battery on the device."]
        #[serde(rename = "batteryThrottlingTemperatures", default)]
        pub battery_throttling_temperatures: Option<Vec<f32>>,
        #[doc = "Brand of the device. For example, Google."]
        #[serde(rename = "brand", default)]
        pub brand: Option<String>,
        #[doc = "CPU shutdown temperature thresholds in Celsius for each CPU on the device."]
        #[serde(rename = "cpuShutdownTemperatures", default)]
        pub cpu_shutdown_temperatures: Option<Vec<f32>>,
        #[doc = "CPU throttling temperature thresholds in Celsius for each CPU on the device."]
        #[serde(rename = "cpuThrottlingTemperatures", default)]
        pub cpu_throttling_temperatures: Option<Vec<f32>>,
        #[doc = "Baseband version. For example, MDM9625_104662.22.05.34p."]
        #[serde(rename = "deviceBasebandVersion", default)]
        pub device_baseband_version: Option<String>,
        #[doc = "GPU shutdown temperature thresholds in Celsius for each GPU on the device."]
        #[serde(rename = "gpuShutdownTemperatures", default)]
        pub gpu_shutdown_temperatures: Option<Vec<f32>>,
        #[doc = "GPU throttling temperature thresholds in Celsius for each GPU on the device."]
        #[serde(rename = "gpuThrottlingTemperatures", default)]
        pub gpu_throttling_temperatures: Option<Vec<f32>>,
        #[doc = "Name of the hardware. For example, Angler."]
        #[serde(rename = "hardware", default)]
        pub hardware: Option<String>,
        #[doc = "Manufacturer. For example, Motorola."]
        #[serde(rename = "manufacturer", default)]
        pub manufacturer: Option<String>,
        #[doc = "The model of the device. For example, Asus Nexus 7."]
        #[serde(rename = "model", default)]
        pub model: Option<String>,
        #[doc = "The device serial number."]
        #[serde(rename = "serialNumber", default)]
        pub serial_number: Option<String>,
        #[doc = "Device skin shutdown temperature thresholds in Celsius."]
        #[serde(rename = "skinShutdownTemperatures", default)]
        pub skin_shutdown_temperatures: Option<Vec<f32>>,
        #[doc = "Device skin throttling temperature thresholds in Celsius."]
        #[serde(rename = "skinThrottlingTemperatures", default)]
        pub skin_throttling_temperatures: Option<Vec<f32>>,
    }
    impl ::field_selector::FieldSelector for HardwareInfo {
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
    pub struct HardwareStatus {
        #[doc = "Current battery temperatures in Celsius for each battery on the device."]
        #[serde(rename = "batteryTemperatures", default)]
        pub battery_temperatures: Option<Vec<f32>>,
        #[doc = "Current CPU temperatures in Celsius for each CPU on the device."]
        #[serde(rename = "cpuTemperatures", default)]
        pub cpu_temperatures: Option<Vec<f32>>,
        #[doc = "CPU usages in percentage for each core available on the device. Usage is 0 for each unplugged core. Empty array implies that CPU usage is not supported in the system."]
        #[serde(rename = "cpuUsages", default)]
        pub cpu_usages: Option<Vec<f32>>,
        #[doc = "The time the measurements were taken."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Fan speeds in RPM for each fan on the device. Empty array means that there are no fans or fan speed is not supported on the system."]
        #[serde(rename = "fanSpeeds", default)]
        pub fan_speeds: Option<Vec<f32>>,
        #[doc = "Current GPU temperatures in Celsius for each GPU on the device."]
        #[serde(rename = "gpuTemperatures", default)]
        pub gpu_temperatures: Option<Vec<f32>>,
        #[doc = "Current device skin temperatures in Celsius."]
        #[serde(rename = "skinTemperatures", default)]
        pub skin_temperatures: Option<Vec<f32>>,
    }
    impl ::field_selector::FieldSelector for HardwareStatus {
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
    pub enum KeyedAppStateSeverity {
        #[doc = "Unspecified severity level."]
        SeverityUnspecified,
        #[doc = "Information severity level."]
        Info,
        #[doc = "Error severity level. This should only be set for genuine error conditions that a management organization needs to take action to fix."]
        Error,
    }
    impl KeyedAppStateSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                KeyedAppStateSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
                KeyedAppStateSeverity::Info => "INFO",
                KeyedAppStateSeverity::Error => "ERROR",
            }
        }
    }
    impl ::std::fmt::Display for KeyedAppStateSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KeyedAppStateSeverity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KeyedAppStateSeverity {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SEVERITY_UNSPECIFIED" => KeyedAppStateSeverity::SeverityUnspecified,
                "INFO" => KeyedAppStateSeverity::Info,
                "ERROR" => KeyedAppStateSeverity::Error,
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
    pub struct KeyedAppState {
        #[doc = "The creation time of the app state on the device."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Optionally, a machine-readable value to be read by the EMM. For example, setting values that the admin can choose to query against in the EMM console (e.g. \u{201c}notify me if the battery_warning data < 10\u{201d})."]
        #[serde(rename = "data", default)]
        pub data: Option<String>,
        #[doc = "The key for the app state. Acts as a point of reference for what the app is providing state for. For example, when providing managed configuration feedback, this key could be the managed configuration key."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "The time the app state was most recently updated."]
        #[serde(rename = "lastUpdateTime", default)]
        pub last_update_time: Option<String>,
        #[doc = "Optionally, a free-form message string to explain the app state. If the state was triggered by a particular value (e.g. a managed configuration value), it should be included in the message."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
        #[doc = "The severity of the app state."]
        #[serde(rename = "severity", default)]
        pub severity: Option<crate::schemas::KeyedAppStateSeverity>,
    }
    impl ::field_selector::FieldSelector for KeyedAppState {
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
    pub struct LaunchAppAction {
        #[doc = "Package name of app to be launched"]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for LaunchAppAction {
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
    pub struct ListDevicesResponse {
        #[doc = "The list of devices."]
        #[serde(rename = "devices", default)]
        pub devices: Option<Vec<crate::schemas::Device>>,
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListDevicesResponse {
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
    pub struct ListPoliciesResponse {
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of policies."]
        #[serde(rename = "policies", default)]
        pub policies: Option<Vec<crate::schemas::Policy>>,
    }
    impl ::field_selector::FieldSelector for ListPoliciesResponse {
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
    pub struct ListWebAppsResponse {
        #[doc = "If there are more results, a token to retrieve next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of web apps."]
        #[serde(rename = "webApps", default)]
        pub web_apps: Option<Vec<crate::schemas::WebApp>>,
    }
    impl ::field_selector::FieldSelector for ListWebAppsResponse {
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
    pub struct ManagedConfigurationTemplate {
        #[doc = "Optional, a map containing <key, value> configuration variables defined for the configuration."]
        #[serde(rename = "configurationVariables", default)]
        pub configuration_variables: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The ID of the managed configurations template."]
        #[serde(rename = "templateId", default)]
        pub template_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ManagedConfigurationTemplate {
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
    pub enum ManagedPropertyType {
        #[doc = "Not used."]
        ManagedPropertyTypeUnspecified,
        #[doc = "A property of boolean type."]
        Bool,
        #[doc = "A property of string type."]
        String,
        #[doc = "A property of integer type."]
        Integer,
        #[doc = "A choice of one item from a set."]
        Choice,
        #[doc = "A choice of multiple items from a set."]
        Multiselect,
        #[doc = "A hidden restriction of string type (the default value can be used to pass along information that can't be modified, such as a version code)."]
        Hidden,
        #[doc = "An array of property bundles."]
        BundleArray,
    }
    impl ManagedPropertyType {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedPropertyType::ManagedPropertyTypeUnspecified => {
                    "MANAGED_PROPERTY_TYPE_UNSPECIFIED"
                }
                ManagedPropertyType::Bool => "BOOL",
                ManagedPropertyType::String => "STRING",
                ManagedPropertyType::Integer => "INTEGER",
                ManagedPropertyType::Choice => "CHOICE",
                ManagedPropertyType::Multiselect => "MULTISELECT",
                ManagedPropertyType::Hidden => "HIDDEN",
                ManagedPropertyType::BundleArray => "BUNDLE_ARRAY",
            }
        }
    }
    impl ::std::fmt::Display for ManagedPropertyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedPropertyType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedPropertyType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MANAGED_PROPERTY_TYPE_UNSPECIFIED" => {
                    ManagedPropertyType::ManagedPropertyTypeUnspecified
                }
                "BOOL" => ManagedPropertyType::Bool,
                "STRING" => ManagedPropertyType::String,
                "INTEGER" => ManagedPropertyType::Integer,
                "CHOICE" => ManagedPropertyType::Choice,
                "MULTISELECT" => ManagedPropertyType::Multiselect,
                "HIDDEN" => ManagedPropertyType::Hidden,
                "BUNDLE_ARRAY" => ManagedPropertyType::BundleArray,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ManagedProperty {
        #[doc = "The default value of the property. BUNDLE_ARRAY properties don't have a default value."]
        #[serde(rename = "defaultValue", default)]
        pub default_value: Option<::serde_json::Value>,
        #[doc = "A longer description of the property, providing more detail of what it affects. Localized."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "For CHOICE or MULTISELECT properties, the list of possible entries."]
        #[serde(rename = "entries", default)]
        pub entries: Option<Vec<crate::schemas::ManagedPropertyEntry>>,
        #[doc = "The unique key that the app uses to identify the property, e.g. \"com.google.android.gm.fieldname\"."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "For BUNDLE_ARRAY properties, the list of nested properties. A BUNDLE_ARRAY property is at most two levels deep."]
        #[serde(rename = "nestedProperties", default)]
        pub nested_properties: Option<Vec<crate::schemas::ManagedProperty>>,
        #[doc = "The type of the property."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::ManagedPropertyType>,
        #[doc = "The name of the property. Localized."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for ManagedProperty {
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
    pub struct ManagedPropertyEntry {
        #[doc = "The human-readable name of the value. Localized."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The machine-readable value of the entry, which should be used in the configuration. Not localized."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for ManagedPropertyEntry {
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
    pub enum MemoryEventEventType {
        #[doc = "Unspecified. No events have this type."]
        MemoryEventTypeUnspecified,
        #[doc = "Free space in RAM was measured."]
        RamMeasured,
        #[doc = "Free space in internal storage was measured."]
        InternalStorageMeasured,
        #[doc = "A new external storage medium was detected. The reported byte count is the total capacity of the storage medium."]
        ExternalStorageDetected,
        #[doc = "An external storage medium was removed. The reported byte count is zero."]
        ExternalStorageRemoved,
        #[doc = "Free space in an external storage medium was measured."]
        ExternalStorageMeasured,
    }
    impl MemoryEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                MemoryEventEventType::MemoryEventTypeUnspecified => "MEMORY_EVENT_TYPE_UNSPECIFIED",
                MemoryEventEventType::RamMeasured => "RAM_MEASURED",
                MemoryEventEventType::InternalStorageMeasured => "INTERNAL_STORAGE_MEASURED",
                MemoryEventEventType::ExternalStorageDetected => "EXTERNAL_STORAGE_DETECTED",
                MemoryEventEventType::ExternalStorageRemoved => "EXTERNAL_STORAGE_REMOVED",
                MemoryEventEventType::ExternalStorageMeasured => "EXTERNAL_STORAGE_MEASURED",
            }
        }
    }
    impl ::std::fmt::Display for MemoryEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MemoryEventEventType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MemoryEventEventType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MEMORY_EVENT_TYPE_UNSPECIFIED" => MemoryEventEventType::MemoryEventTypeUnspecified,
                "RAM_MEASURED" => MemoryEventEventType::RamMeasured,
                "INTERNAL_STORAGE_MEASURED" => MemoryEventEventType::InternalStorageMeasured,
                "EXTERNAL_STORAGE_DETECTED" => MemoryEventEventType::ExternalStorageDetected,
                "EXTERNAL_STORAGE_REMOVED" => MemoryEventEventType::ExternalStorageRemoved,
                "EXTERNAL_STORAGE_MEASURED" => MemoryEventEventType::ExternalStorageMeasured,
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
    pub struct MemoryEvent {
        #[doc = "The number of free bytes in the medium, or for EXTERNAL_STORAGE_DETECTED, the total capacity in bytes of the storage medium."]
        #[serde(rename = "byteCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub byte_count: Option<i64>,
        #[doc = "The creation time of the event."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Event type."]
        #[serde(rename = "eventType", default)]
        pub event_type: Option<crate::schemas::MemoryEventEventType>,
    }
    impl ::field_selector::FieldSelector for MemoryEvent {
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
    pub struct MemoryInfo {
        #[doc = "Total internal storage on device in bytes."]
        #[serde(rename = "totalInternalStorage", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_internal_storage: Option<i64>,
        #[doc = "Total RAM on device in bytes."]
        #[serde(rename = "totalRam", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_ram: Option<i64>,
    }
    impl ::field_selector::FieldSelector for MemoryInfo {
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
    pub struct NetworkInfo {
        #[doc = "IMEI number of the GSM device. For example, A1000031212."]
        #[serde(rename = "imei", default)]
        pub imei: Option<String>,
        #[doc = "MEID number of the CDMA device. For example, A00000292788E1."]
        #[serde(rename = "meid", default)]
        pub meid: Option<String>,
        #[doc = "Alphabetic name of current registered operator. For example, Vodafone."]
        #[serde(rename = "networkOperatorName", default)]
        pub network_operator_name: Option<String>,
        #[doc = "Wi-Fi MAC address of the device. For example, 7c:11:11:11:11:11."]
        #[serde(rename = "wifiMacAddress", default)]
        pub wifi_mac_address: Option<String>,
    }
    impl ::field_selector::FieldSelector for NetworkInfo {
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
    pub enum NonComplianceDetailInstallationFailureReason {
        #[doc = "This value is disallowed."]
        InstallationFailureReasonUnspecified,
        #[doc = "An unknown condition is preventing the app from being installed. Some potential reasons are that the device doesn't have enough storage, the device network connection is unreliable, or the installation is taking longer than expected. The installation will be retried automatically."]
        InstallationFailureReasonUnknown,
        #[doc = "The installation is still in progress."]
        InProgress,
        #[doc = "The app was not found in Play."]
        NotFound,
        #[doc = "The app is incompatible with the device."]
        NotCompatibleWithDevice,
        #[doc = "The app has not been approved by the admin."]
        NotApproved,
        #[doc = "The app has new permissions that have not been accepted by the admin."]
        PermissionsNotAccepted,
        #[doc = "The app is not available in the user's country."]
        NotAvailableInCountry,
        #[doc = "There are no licenses available to assign to the user."]
        NoLicensesRemaining,
        #[doc = "The enterprise is no longer enrolled with managed Play or the admin has not accepted the latest managed Play terms of service."]
        NotEnrolled,
        #[doc = "The user is no longer valid. The user may have been deleted or disabled."]
        UserInvalid,
    }
    impl NonComplianceDetailInstallationFailureReason {
        pub fn as_str(self) -> &'static str {
            match self { NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnspecified => "INSTALLATION_FAILURE_REASON_UNSPECIFIED" , NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnknown => "INSTALLATION_FAILURE_REASON_UNKNOWN" , NonComplianceDetailInstallationFailureReason :: InProgress => "IN_PROGRESS" , NonComplianceDetailInstallationFailureReason :: NotFound => "NOT_FOUND" , NonComplianceDetailInstallationFailureReason :: NotCompatibleWithDevice => "NOT_COMPATIBLE_WITH_DEVICE" , NonComplianceDetailInstallationFailureReason :: NotApproved => "NOT_APPROVED" , NonComplianceDetailInstallationFailureReason :: PermissionsNotAccepted => "PERMISSIONS_NOT_ACCEPTED" , NonComplianceDetailInstallationFailureReason :: NotAvailableInCountry => "NOT_AVAILABLE_IN_COUNTRY" , NonComplianceDetailInstallationFailureReason :: NoLicensesRemaining => "NO_LICENSES_REMAINING" , NonComplianceDetailInstallationFailureReason :: NotEnrolled => "NOT_ENROLLED" , NonComplianceDetailInstallationFailureReason :: UserInvalid => "USER_INVALID" , }
        }
    }
    impl ::std::fmt::Display for NonComplianceDetailInstallationFailureReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonComplianceDetailInstallationFailureReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonComplianceDetailInstallationFailureReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "INSTALLATION_FAILURE_REASON_UNSPECIFIED" => NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnspecified , "INSTALLATION_FAILURE_REASON_UNKNOWN" => NonComplianceDetailInstallationFailureReason :: InstallationFailureReasonUnknown , "IN_PROGRESS" => NonComplianceDetailInstallationFailureReason :: InProgress , "NOT_FOUND" => NonComplianceDetailInstallationFailureReason :: NotFound , "NOT_COMPATIBLE_WITH_DEVICE" => NonComplianceDetailInstallationFailureReason :: NotCompatibleWithDevice , "NOT_APPROVED" => NonComplianceDetailInstallationFailureReason :: NotApproved , "PERMISSIONS_NOT_ACCEPTED" => NonComplianceDetailInstallationFailureReason :: PermissionsNotAccepted , "NOT_AVAILABLE_IN_COUNTRY" => NonComplianceDetailInstallationFailureReason :: NotAvailableInCountry , "NO_LICENSES_REMAINING" => NonComplianceDetailInstallationFailureReason :: NoLicensesRemaining , "NOT_ENROLLED" => NonComplianceDetailInstallationFailureReason :: NotEnrolled , "USER_INVALID" => NonComplianceDetailInstallationFailureReason :: UserInvalid , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NonComplianceDetailNonComplianceReason {
        #[doc = "This value is disallowed."]
        NonComplianceReasonUnspecified,
        #[doc = "The setting is not supported in the API level of the Android version running on the device."]
        ApiLevel,
        #[doc = "The management mode (profile owner, device owner, etc.) doesn't support the setting."]
        ManagementMode,
        #[doc = "The user has not taken required action to comply with the setting."]
        UserAction,
        #[doc = "The setting has an invalid value."]
        InvalidValue,
        #[doc = "The app required to implement the policy is not installed."]
        AppNotInstalled,
        #[doc = "The policy is not supported by the version of Android Device Policy on the device."]
        Unsupported,
        #[doc = "A blocked app is installed."]
        AppInstalled,
        #[doc = "The setting hasn't been applied at the time of the report, but is expected to be applied shortly."]
        Pending,
        #[doc = "The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough."]
        AppIncompatible,
        #[doc = "The app is installed, but it hasn't been updated to the minimum version code specified by policy."]
        AppNotUpdated,
    }
    impl NonComplianceDetailNonComplianceReason {
        pub fn as_str(self) -> &'static str {
            match self {
                NonComplianceDetailNonComplianceReason::NonComplianceReasonUnspecified => {
                    "NON_COMPLIANCE_REASON_UNSPECIFIED"
                }
                NonComplianceDetailNonComplianceReason::ApiLevel => "API_LEVEL",
                NonComplianceDetailNonComplianceReason::ManagementMode => "MANAGEMENT_MODE",
                NonComplianceDetailNonComplianceReason::UserAction => "USER_ACTION",
                NonComplianceDetailNonComplianceReason::InvalidValue => "INVALID_VALUE",
                NonComplianceDetailNonComplianceReason::AppNotInstalled => "APP_NOT_INSTALLED",
                NonComplianceDetailNonComplianceReason::Unsupported => "UNSUPPORTED",
                NonComplianceDetailNonComplianceReason::AppInstalled => "APP_INSTALLED",
                NonComplianceDetailNonComplianceReason::Pending => "PENDING",
                NonComplianceDetailNonComplianceReason::AppIncompatible => "APP_INCOMPATIBLE",
                NonComplianceDetailNonComplianceReason::AppNotUpdated => "APP_NOT_UPDATED",
            }
        }
    }
    impl ::std::fmt::Display for NonComplianceDetailNonComplianceReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonComplianceDetailNonComplianceReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonComplianceDetailNonComplianceReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NON_COMPLIANCE_REASON_UNSPECIFIED" => {
                    NonComplianceDetailNonComplianceReason::NonComplianceReasonUnspecified
                }
                "API_LEVEL" => NonComplianceDetailNonComplianceReason::ApiLevel,
                "MANAGEMENT_MODE" => NonComplianceDetailNonComplianceReason::ManagementMode,
                "USER_ACTION" => NonComplianceDetailNonComplianceReason::UserAction,
                "INVALID_VALUE" => NonComplianceDetailNonComplianceReason::InvalidValue,
                "APP_NOT_INSTALLED" => NonComplianceDetailNonComplianceReason::AppNotInstalled,
                "UNSUPPORTED" => NonComplianceDetailNonComplianceReason::Unsupported,
                "APP_INSTALLED" => NonComplianceDetailNonComplianceReason::AppInstalled,
                "PENDING" => NonComplianceDetailNonComplianceReason::Pending,
                "APP_INCOMPATIBLE" => NonComplianceDetailNonComplianceReason::AppIncompatible,
                "APP_NOT_UPDATED" => NonComplianceDetailNonComplianceReason::AppNotUpdated,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct NonComplianceDetail {
        #[doc = "If the policy setting could not be applied, the current value of the setting on the device."]
        #[serde(rename = "currentValue", default)]
        pub current_value: Option<::serde_json::Value>,
        #[doc = "For settings with nested fields, if a particular nested field is out of compliance, this specifies the full path to the offending field. The path is formatted in the same way the policy JSON field would be referenced in JavaScript, that is: 1) For object-typed fields, the field name is followed by a dot then by a  subfield name. 2) For array-typed fields, the field name is followed by the array index  enclosed in brackets. For example, to indicate a problem with the url field in the externalData field in the 3rd application, the path would be applications[2].externalData.url"]
        #[serde(rename = "fieldPath", default)]
        pub field_path: Option<String>,
        #[doc = "If package_name is set and the non-compliance reason is APP_NOT_INSTALLED or APP_NOT_UPDATED, the detailed reason the app can't be installed or updated."]
        #[serde(rename = "installationFailureReason", default)]
        pub installation_failure_reason:
            Option<crate::schemas::NonComplianceDetailInstallationFailureReason>,
        #[doc = "The reason the device is not in compliance with the setting."]
        #[serde(rename = "nonComplianceReason", default)]
        pub non_compliance_reason: Option<crate::schemas::NonComplianceDetailNonComplianceReason>,
        #[doc = "The package name indicating which app is out of compliance, if applicable."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
        #[doc = "The name of the policy setting. This is the JSON field name of a top-level Policy  field."]
        #[serde(rename = "settingName", default)]
        pub setting_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for NonComplianceDetail {
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
    pub enum NonComplianceDetailConditionNonComplianceReason {
        #[doc = "This value is disallowed."]
        NonComplianceReasonUnspecified,
        #[doc = "The setting is not supported in the API level of the Android version running on the device."]
        ApiLevel,
        #[doc = "The management mode (profile owner, device owner, etc.) doesn't support the setting."]
        ManagementMode,
        #[doc = "The user has not taken required action to comply with the setting."]
        UserAction,
        #[doc = "The setting has an invalid value."]
        InvalidValue,
        #[doc = "The app required to implement the policy is not installed."]
        AppNotInstalled,
        #[doc = "The policy is not supported by the version of Android Device Policy on the device."]
        Unsupported,
        #[doc = "A blocked app is installed."]
        AppInstalled,
        #[doc = "The setting hasn't been applied at the time of the report, but is expected to be applied shortly."]
        Pending,
        #[doc = "The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough."]
        AppIncompatible,
        #[doc = "The app is installed, but it hasn't been updated to the minimum version code specified by policy."]
        AppNotUpdated,
    }
    impl NonComplianceDetailConditionNonComplianceReason {
        pub fn as_str(self) -> &'static str {
            match self {
                NonComplianceDetailConditionNonComplianceReason::NonComplianceReasonUnspecified => {
                    "NON_COMPLIANCE_REASON_UNSPECIFIED"
                }
                NonComplianceDetailConditionNonComplianceReason::ApiLevel => "API_LEVEL",
                NonComplianceDetailConditionNonComplianceReason::ManagementMode => {
                    "MANAGEMENT_MODE"
                }
                NonComplianceDetailConditionNonComplianceReason::UserAction => "USER_ACTION",
                NonComplianceDetailConditionNonComplianceReason::InvalidValue => "INVALID_VALUE",
                NonComplianceDetailConditionNonComplianceReason::AppNotInstalled => {
                    "APP_NOT_INSTALLED"
                }
                NonComplianceDetailConditionNonComplianceReason::Unsupported => "UNSUPPORTED",
                NonComplianceDetailConditionNonComplianceReason::AppInstalled => "APP_INSTALLED",
                NonComplianceDetailConditionNonComplianceReason::Pending => "PENDING",
                NonComplianceDetailConditionNonComplianceReason::AppIncompatible => {
                    "APP_INCOMPATIBLE"
                }
                NonComplianceDetailConditionNonComplianceReason::AppNotUpdated => "APP_NOT_UPDATED",
            }
        }
    }
    impl ::std::fmt::Display for NonComplianceDetailConditionNonComplianceReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NonComplianceDetailConditionNonComplianceReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NonComplianceDetailConditionNonComplianceReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NON_COMPLIANCE_REASON_UNSPECIFIED" => {
                    NonComplianceDetailConditionNonComplianceReason::NonComplianceReasonUnspecified
                }
                "API_LEVEL" => NonComplianceDetailConditionNonComplianceReason::ApiLevel,
                "MANAGEMENT_MODE" => {
                    NonComplianceDetailConditionNonComplianceReason::ManagementMode
                }
                "USER_ACTION" => NonComplianceDetailConditionNonComplianceReason::UserAction,
                "INVALID_VALUE" => NonComplianceDetailConditionNonComplianceReason::InvalidValue,
                "APP_NOT_INSTALLED" => {
                    NonComplianceDetailConditionNonComplianceReason::AppNotInstalled
                }
                "UNSUPPORTED" => NonComplianceDetailConditionNonComplianceReason::Unsupported,
                "APP_INSTALLED" => NonComplianceDetailConditionNonComplianceReason::AppInstalled,
                "PENDING" => NonComplianceDetailConditionNonComplianceReason::Pending,
                "APP_INCOMPATIBLE" => {
                    NonComplianceDetailConditionNonComplianceReason::AppIncompatible
                }
                "APP_NOT_UPDATED" => NonComplianceDetailConditionNonComplianceReason::AppNotUpdated,
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
    pub struct NonComplianceDetailCondition {
        #[doc = "The reason the device is not in compliance with the setting. If not set, then this condition matches any reason."]
        #[serde(rename = "nonComplianceReason", default)]
        pub non_compliance_reason:
            Option<crate::schemas::NonComplianceDetailConditionNonComplianceReason>,
        #[doc = "The package name of the app that's out of compliance. If not set, then this condition matches any package name."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
        #[doc = "The name of the policy setting. This is the JSON field name of a top-level Policy field. If not set, then this condition matches any setting name."]
        #[serde(rename = "settingName", default)]
        pub setting_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for NonComplianceDetailCondition {
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
        #[doc = "If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse."]
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
    pub struct PackageNameList {
        #[doc = "A list of package names."]
        #[serde(rename = "packageNames", default)]
        pub package_names: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for PackageNameList {
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
    pub enum PasswordRequirementsPasswordQuality {
        #[doc = "There are no password requirements."]
        PasswordQualityUnspecified,
        #[doc = "The device must be secured with a low-security biometric recognition technology, at minimum. This includes technologies that can recognize the identity of an individual that are roughly equivalent to a 3-digit PIN (false detection is less than 1 in 1,000)."]
        BiometricWeak,
        #[doc = "A password is required, but there are no restrictions on what the password must contain."]
        Something,
        #[doc = "The password must contain numeric characters."]
        Numeric,
        #[doc = "The password must contain numeric characters with no repeating (4444) or ordered (1234, 4321, 2468) sequences."]
        NumericComplex,
        #[doc = "The password must contain alphabetic (or symbol) characters."]
        Alphabetic,
        #[doc = "The password must contain both numeric and alphabetic (or symbol) characters."]
        Alphanumeric,
        #[doc = "The password must contain at least a letter, a numerical digit and a special symbol. Other password constraints, for example, password_minimum_letters are enforced."]
        Complex,
    }
    impl PasswordRequirementsPasswordQuality {
        pub fn as_str(self) -> &'static str {
            match self {
                PasswordRequirementsPasswordQuality::PasswordQualityUnspecified => {
                    "PASSWORD_QUALITY_UNSPECIFIED"
                }
                PasswordRequirementsPasswordQuality::BiometricWeak => "BIOMETRIC_WEAK",
                PasswordRequirementsPasswordQuality::Something => "SOMETHING",
                PasswordRequirementsPasswordQuality::Numeric => "NUMERIC",
                PasswordRequirementsPasswordQuality::NumericComplex => "NUMERIC_COMPLEX",
                PasswordRequirementsPasswordQuality::Alphabetic => "ALPHABETIC",
                PasswordRequirementsPasswordQuality::Alphanumeric => "ALPHANUMERIC",
                PasswordRequirementsPasswordQuality::Complex => "COMPLEX",
            }
        }
    }
    impl ::std::fmt::Display for PasswordRequirementsPasswordQuality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PasswordRequirementsPasswordQuality {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PasswordRequirementsPasswordQuality {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PASSWORD_QUALITY_UNSPECIFIED" => {
                    PasswordRequirementsPasswordQuality::PasswordQualityUnspecified
                }
                "BIOMETRIC_WEAK" => PasswordRequirementsPasswordQuality::BiometricWeak,
                "SOMETHING" => PasswordRequirementsPasswordQuality::Something,
                "NUMERIC" => PasswordRequirementsPasswordQuality::Numeric,
                "NUMERIC_COMPLEX" => PasswordRequirementsPasswordQuality::NumericComplex,
                "ALPHABETIC" => PasswordRequirementsPasswordQuality::Alphabetic,
                "ALPHANUMERIC" => PasswordRequirementsPasswordQuality::Alphanumeric,
                "COMPLEX" => PasswordRequirementsPasswordQuality::Complex,
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
    pub enum PasswordRequirementsPasswordScope {
        #[doc = "The scope is unspecified. The password requirements are applied to the work profile for work profile devices and the whole device for fully managed or dedicated devices."]
        ScopeUnspecified,
        #[doc = "The password requirements are only applied to the device."]
        ScopeDevice,
        #[doc = "The password requirements are only applied to the work profile."]
        ScopeProfile,
    }
    impl PasswordRequirementsPasswordScope {
        pub fn as_str(self) -> &'static str {
            match self {
                PasswordRequirementsPasswordScope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
                PasswordRequirementsPasswordScope::ScopeDevice => "SCOPE_DEVICE",
                PasswordRequirementsPasswordScope::ScopeProfile => "SCOPE_PROFILE",
            }
        }
    }
    impl ::std::fmt::Display for PasswordRequirementsPasswordScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PasswordRequirementsPasswordScope {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PasswordRequirementsPasswordScope {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_UNSPECIFIED" => PasswordRequirementsPasswordScope::ScopeUnspecified,
                "SCOPE_DEVICE" => PasswordRequirementsPasswordScope::ScopeDevice,
                "SCOPE_PROFILE" => PasswordRequirementsPasswordScope::ScopeProfile,
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
    pub struct PasswordRequirements {
        #[doc = "Number of incorrect device-unlock passwords that can be entered before a device is wiped. A value of 0 means there is no restriction."]
        #[serde(rename = "maximumFailedPasswordsForWipe", default)]
        pub maximum_failed_passwords_for_wipe: Option<i32>,
        #[doc = "Password expiration timeout."]
        #[serde(rename = "passwordExpirationTimeout", default)]
        pub password_expiration_timeout: Option<String>,
        #[doc = "The length of the password history. After setting this field, the user won't be able to enter a new password that is the same as any password in the history. A value of 0 means there is no restriction."]
        #[serde(rename = "passwordHistoryLength", default)]
        pub password_history_length: Option<i32>,
        #[doc = "The minimum allowed password length. A value of 0 means there is no restriction. Only enforced when password_quality is NUMERIC, NUMERIC_COMPLEX, ALPHABETIC, ALPHANUMERIC, or COMPLEX."]
        #[serde(rename = "passwordMinimumLength", default)]
        pub password_minimum_length: Option<i32>,
        #[doc = "Minimum number of letters required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(rename = "passwordMinimumLetters", default)]
        pub password_minimum_letters: Option<i32>,
        #[doc = "Minimum number of lower case letters required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(rename = "passwordMinimumLowerCase", default)]
        pub password_minimum_lower_case: Option<i32>,
        #[doc = "Minimum number of non-letter characters (numerical digits or symbols) required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(rename = "passwordMinimumNonLetter", default)]
        pub password_minimum_non_letter: Option<i32>,
        #[doc = "Minimum number of numerical digits required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(rename = "passwordMinimumNumeric", default)]
        pub password_minimum_numeric: Option<i32>,
        #[doc = "Minimum number of symbols required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(rename = "passwordMinimumSymbols", default)]
        pub password_minimum_symbols: Option<i32>,
        #[doc = "Minimum number of upper case letters required in the password. Only enforced when password_quality is COMPLEX."]
        #[serde(rename = "passwordMinimumUpperCase", default)]
        pub password_minimum_upper_case: Option<i32>,
        #[doc = "The required password quality."]
        #[serde(rename = "passwordQuality", default)]
        pub password_quality: Option<crate::schemas::PasswordRequirementsPasswordQuality>,
        #[doc = "The scope that the password requirement applies to."]
        #[serde(rename = "passwordScope", default)]
        pub password_scope: Option<crate::schemas::PasswordRequirementsPasswordScope>,
    }
    impl ::field_selector::FieldSelector for PasswordRequirements {
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
    pub enum PermissionGrantPolicy {
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
        #[doc = "Automatically grant a permission."]
        Grant,
        #[doc = "Automatically deny a permission."]
        Deny,
    }
    impl PermissionGrantPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PermissionGrantPolicy::PermissionPolicyUnspecified => {
                    "PERMISSION_POLICY_UNSPECIFIED"
                }
                PermissionGrantPolicy::Prompt => "PROMPT",
                PermissionGrantPolicy::Grant => "GRANT",
                PermissionGrantPolicy::Deny => "DENY",
            }
        }
    }
    impl ::std::fmt::Display for PermissionGrantPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PermissionGrantPolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PermissionGrantPolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    PermissionGrantPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => PermissionGrantPolicy::Prompt,
                "GRANT" => PermissionGrantPolicy::Grant,
                "DENY" => PermissionGrantPolicy::Deny,
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
    pub struct PermissionGrant {
        #[doc = "The Android permission or group, e.g. android.permission.READ_CALENDAR or android.permission_group.CALENDAR."]
        #[serde(rename = "permission", default)]
        pub permission: Option<String>,
        #[doc = "The policy for granting the permission."]
        #[serde(rename = "policy", default)]
        pub policy: Option<crate::schemas::PermissionGrantPolicy>,
    }
    impl ::field_selector::FieldSelector for PermissionGrant {
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
    pub struct PersistentPreferredActivity {
        #[doc = "The intent actions to match in the filter. If any actions are included in the filter, then an intent's action must be one of those values for it to match. If no actions are included, the intent action is ignored."]
        #[serde(rename = "actions", default)]
        pub actions: Option<Vec<String>>,
        #[doc = "The intent categories to match in the filter. An intent includes the categories that it requires, all of which must be included in the filter in order to match. In other words, adding a category to the filter has no impact on matching unless that category is specified in the intent."]
        #[serde(rename = "categories", default)]
        pub categories: Option<Vec<String>>,
        #[doc = "The activity that should be the default intent handler. This should be an Android component name, e.g. com.android.enterprise.app/.MainActivity. Alternatively, the value may be the package name of an app, which causes Android Device Policy to choose an appropriate activity from the app to handle the intent."]
        #[serde(rename = "receiverActivity", default)]
        pub receiver_activity: Option<String>,
    }
    impl ::field_selector::FieldSelector for PersistentPreferredActivity {
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
    pub enum PolicyAndroidDevicePolicyTracksItems {}
    impl PolicyAndroidDevicePolicyTracksItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for PolicyAndroidDevicePolicyTracksItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyAndroidDevicePolicyTracksItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyAndroidDevicePolicyTracksItems {
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyAppAutoUpdatePolicy {
        #[doc = "The auto-update policy is not set. Equivalent to CHOICE_TO_THE_USER."]
        AppAutoUpdatePolicyUnspecified,
        #[doc = "The user can control auto-updates."]
        ChoiceToTheUser,
        #[doc = "Apps are never auto-updated."]
        Never,
        #[doc = "Apps are auto-updated over Wi-Fi only."]
        WifiOnly,
        #[doc = "Apps are auto-updated at any time. Data charges may apply."]
        Always,
    }
    impl PolicyAppAutoUpdatePolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyAppAutoUpdatePolicy::AppAutoUpdatePolicyUnspecified => {
                    "APP_AUTO_UPDATE_POLICY_UNSPECIFIED"
                }
                PolicyAppAutoUpdatePolicy::ChoiceToTheUser => "CHOICE_TO_THE_USER",
                PolicyAppAutoUpdatePolicy::Never => "NEVER",
                PolicyAppAutoUpdatePolicy::WifiOnly => "WIFI_ONLY",
                PolicyAppAutoUpdatePolicy::Always => "ALWAYS",
            }
        }
    }
    impl ::std::fmt::Display for PolicyAppAutoUpdatePolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyAppAutoUpdatePolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyAppAutoUpdatePolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_AUTO_UPDATE_POLICY_UNSPECIFIED" => {
                    PolicyAppAutoUpdatePolicy::AppAutoUpdatePolicyUnspecified
                }
                "CHOICE_TO_THE_USER" => PolicyAppAutoUpdatePolicy::ChoiceToTheUser,
                "NEVER" => PolicyAppAutoUpdatePolicy::Never,
                "WIFI_ONLY" => PolicyAppAutoUpdatePolicy::WifiOnly,
                "ALWAYS" => PolicyAppAutoUpdatePolicy::Always,
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
    pub enum PolicyDefaultPermissionPolicy {
        #[doc = "Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default."]
        PermissionPolicyUnspecified,
        #[doc = "Prompt the user to grant a permission."]
        Prompt,
        #[doc = "Automatically grant a permission."]
        Grant,
        #[doc = "Automatically deny a permission."]
        Deny,
    }
    impl PolicyDefaultPermissionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyDefaultPermissionPolicy::PermissionPolicyUnspecified => {
                    "PERMISSION_POLICY_UNSPECIFIED"
                }
                PolicyDefaultPermissionPolicy::Prompt => "PROMPT",
                PolicyDefaultPermissionPolicy::Grant => "GRANT",
                PolicyDefaultPermissionPolicy::Deny => "DENY",
            }
        }
    }
    impl ::std::fmt::Display for PolicyDefaultPermissionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyDefaultPermissionPolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyDefaultPermissionPolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERMISSION_POLICY_UNSPECIFIED" => {
                    PolicyDefaultPermissionPolicy::PermissionPolicyUnspecified
                }
                "PROMPT" => PolicyDefaultPermissionPolicy::Prompt,
                "GRANT" => PolicyDefaultPermissionPolicy::Grant,
                "DENY" => PolicyDefaultPermissionPolicy::Deny,
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
    pub enum PolicyEncryptionPolicy {
        #[doc = "This value is ignored, i.e. no encryption required"]
        EncryptionPolicyUnspecified,
        #[doc = "Encryption required but no password required to boot"]
        EnabledWithoutPassword,
        #[doc = "Encryption required with password required to boot"]
        EnabledWithPassword,
    }
    impl PolicyEncryptionPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyEncryptionPolicy::EncryptionPolicyUnspecified => {
                    "ENCRYPTION_POLICY_UNSPECIFIED"
                }
                PolicyEncryptionPolicy::EnabledWithoutPassword => "ENABLED_WITHOUT_PASSWORD",
                PolicyEncryptionPolicy::EnabledWithPassword => "ENABLED_WITH_PASSWORD",
            }
        }
    }
    impl ::std::fmt::Display for PolicyEncryptionPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyEncryptionPolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyEncryptionPolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENCRYPTION_POLICY_UNSPECIFIED" => {
                    PolicyEncryptionPolicy::EncryptionPolicyUnspecified
                }
                "ENABLED_WITHOUT_PASSWORD" => PolicyEncryptionPolicy::EnabledWithoutPassword,
                "ENABLED_WITH_PASSWORD" => PolicyEncryptionPolicy::EnabledWithPassword,
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
    pub enum PolicyKeyguardDisabledFeaturesItems {}
    impl PolicyKeyguardDisabledFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for PolicyKeyguardDisabledFeaturesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyKeyguardDisabledFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyKeyguardDisabledFeaturesItems {
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyLocationMode {
        #[doc = "The current device value is not modified."]
        LocationModeUnspecified,
        #[doc = "All location detection methods are enabled, including GPS, networks, and other sensors."]
        HighAccuracy,
        #[doc = "Only GPS and other sensors are enabled."]
        SensorsOnly,
        #[doc = "Only the network location provider is enabled."]
        BatterySaving,
        #[doc = "Location detection is disabled."]
        Off,
    }
    impl PolicyLocationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyLocationMode::LocationModeUnspecified => "LOCATION_MODE_UNSPECIFIED",
                PolicyLocationMode::HighAccuracy => "HIGH_ACCURACY",
                PolicyLocationMode::SensorsOnly => "SENSORS_ONLY",
                PolicyLocationMode::BatterySaving => "BATTERY_SAVING",
                PolicyLocationMode::Off => "OFF",
            }
        }
    }
    impl ::std::fmt::Display for PolicyLocationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyLocationMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyLocationMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOCATION_MODE_UNSPECIFIED" => PolicyLocationMode::LocationModeUnspecified,
                "HIGH_ACCURACY" => PolicyLocationMode::HighAccuracy,
                "SENSORS_ONLY" => PolicyLocationMode::SensorsOnly,
                "BATTERY_SAVING" => PolicyLocationMode::BatterySaving,
                "OFF" => PolicyLocationMode::Off,
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
    pub enum PolicyPlayStoreMode {
        #[doc = "Unspecified. Defaults to WHITELIST."]
        PlayStoreModeUnspecified,
        #[doc = "Only apps that are in the policy are available and any app not in the policy will be automatically uninstalled from the device."]
        Whitelist,
        #[doc = "All apps are available and any app that should not be on the device should be explicitly marked as 'BLOCKED' in the applications policy."]
        Blacklist,
    }
    impl PolicyPlayStoreMode {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyPlayStoreMode::PlayStoreModeUnspecified => "PLAY_STORE_MODE_UNSPECIFIED",
                PolicyPlayStoreMode::Whitelist => "WHITELIST",
                PolicyPlayStoreMode::Blacklist => "BLACKLIST",
            }
        }
    }
    impl ::std::fmt::Display for PolicyPlayStoreMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyPlayStoreMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyPlayStoreMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PLAY_STORE_MODE_UNSPECIFIED" => PolicyPlayStoreMode::PlayStoreModeUnspecified,
                "WHITELIST" => PolicyPlayStoreMode::Whitelist,
                "BLACKLIST" => PolicyPlayStoreMode::Blacklist,
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
    pub enum PolicyStayOnPluggedModesItems {}
    impl PolicyStayOnPluggedModesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for PolicyStayOnPluggedModesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyStayOnPluggedModesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyStayOnPluggedModesItems {
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Policy {
        #[doc = "Account types that can't be managed by the user."]
        #[serde(rename = "accountTypesWithManagementDisabled", default)]
        pub account_types_with_management_disabled: Option<Vec<String>>,
        #[doc = "Whether adding new users and profiles is disabled."]
        #[serde(rename = "addUserDisabled", default)]
        pub add_user_disabled: Option<bool>,
        #[doc = "Whether adjusting the master volume is disabled."]
        #[serde(rename = "adjustVolumeDisabled", default)]
        pub adjust_volume_disabled: Option<bool>,
        #[doc = "Configuration for an always-on VPN connection. Use with vpn_config_disabled to prevent modification of this setting."]
        #[serde(rename = "alwaysOnVpnPackage", default)]
        pub always_on_vpn_package: Option<crate::schemas::AlwaysOnVpnPackage>,
        #[doc = "The app tracks for Android Device Policy the device can access. The device receives the latest version among all accessible tracks. If no tracks are specified, then the device only uses the production track."]
        #[serde(rename = "androidDevicePolicyTracks", default)]
        pub android_device_policy_tracks:
            Option<Vec<crate::schemas::PolicyAndroidDevicePolicyTracksItems>>,
        #[doc = "The app auto update policy, which controls when automatic app updates can be applied."]
        #[serde(rename = "appAutoUpdatePolicy", default)]
        pub app_auto_update_policy: Option<crate::schemas::PolicyAppAutoUpdatePolicy>,
        #[doc = "Policy applied to apps."]
        #[serde(rename = "applications", default)]
        pub applications: Option<Vec<crate::schemas::ApplicationPolicy>>,
        #[doc = "Whether auto time is required, which prevents the user from manually setting the date and time."]
        #[serde(rename = "autoTimeRequired", default)]
        pub auto_time_required: Option<bool>,
        #[doc = "Whether applications other than the ones configured in applications are blocked from being installed. When set, applications that were installed under a previous policy but no longer appear in the policy are automatically uninstalled."]
        #[serde(rename = "blockApplicationsEnabled", default)]
        pub block_applications_enabled: Option<bool>,
        #[doc = "Whether configuring bluetooth is disabled."]
        #[serde(rename = "bluetoothConfigDisabled", default)]
        pub bluetooth_config_disabled: Option<bool>,
        #[doc = "Whether bluetooth contact sharing is disabled."]
        #[serde(rename = "bluetoothContactSharingDisabled", default)]
        pub bluetooth_contact_sharing_disabled: Option<bool>,
        #[doc = "Whether bluetooth is disabled. Prefer this setting over bluetooth_config_disabled because bluetooth_config_disabled can be bypassed by the user."]
        #[serde(rename = "bluetoothDisabled", default)]
        pub bluetooth_disabled: Option<bool>,
        #[doc = "Whether all cameras on the device are disabled."]
        #[serde(rename = "cameraDisabled", default)]
        pub camera_disabled: Option<bool>,
        #[doc = "Whether configuring cell broadcast is disabled."]
        #[serde(rename = "cellBroadcastsConfigDisabled", default)]
        pub cell_broadcasts_config_disabled: Option<bool>,
        #[doc = "Rules for automatically choosing a private key and certificate to authenticate the device to a server. The rules are ordered by increasing precedence, so if an outgoing request matches more than one rule, the last rule defines which private key to use."]
        #[serde(rename = "choosePrivateKeyRules", default)]
        pub choose_private_key_rules: Option<Vec<crate::schemas::ChoosePrivateKeyRule>>,
        #[doc = "Rules declaring which mitigating actions to take when a device is not compliant with its policy. When the conditions for multiple rules are satisfied, all of the mitigating actions for the rules are taken. There is a maximum limit of 100 rules. Use policy enforcement rules instead."]
        #[serde(rename = "complianceRules", default)]
        pub compliance_rules: Option<Vec<crate::schemas::ComplianceRule>>,
        #[doc = "Whether creating windows besides app windows is disabled."]
        #[serde(rename = "createWindowsDisabled", default)]
        pub create_windows_disabled: Option<bool>,
        #[doc = "Whether configuring user credentials is disabled."]
        #[serde(rename = "credentialsConfigDisabled", default)]
        pub credentials_config_disabled: Option<bool>,
        #[doc = "Whether roaming data services are disabled."]
        #[serde(rename = "dataRoamingDisabled", default)]
        pub data_roaming_disabled: Option<bool>,
        #[doc = "Whether the user is allowed to enable debugging features."]
        #[serde(rename = "debuggingFeaturesAllowed", default)]
        pub debugging_features_allowed: Option<bool>,
        #[doc = "The default permission policy for runtime permission requests."]
        #[serde(rename = "defaultPermissionPolicy", default)]
        pub default_permission_policy: Option<crate::schemas::PolicyDefaultPermissionPolicy>,
        #[doc = "The device owner information to be shown on the lock screen."]
        #[serde(rename = "deviceOwnerLockScreenInfo", default)]
        pub device_owner_lock_screen_info: Option<crate::schemas::UserFacingMessage>,
        #[doc = "Whether encryption is enabled"]
        #[serde(rename = "encryptionPolicy", default)]
        pub encryption_policy: Option<crate::schemas::PolicyEncryptionPolicy>,
        #[doc = "Whether app verification is force-enabled."]
        #[serde(rename = "ensureVerifyAppsEnabled", default)]
        pub ensure_verify_apps_enabled: Option<bool>,
        #[doc = "Whether factory resetting from settings is disabled."]
        #[serde(rename = "factoryResetDisabled", default)]
        pub factory_reset_disabled: Option<bool>,
        #[doc = "Email addresses of device administrators for factory reset protection. When the device is factory reset, it will require one of these admins to log in with the Google account email and password to unlock the device. If no admins are specified, the device won't provide factory reset protection."]
        #[serde(rename = "frpAdminEmails", default)]
        pub frp_admin_emails: Option<Vec<String>>,
        #[doc = "Whether the user is allowed to have fun. Controls whether the Easter egg game in Settings is disabled."]
        #[serde(rename = "funDisabled", default)]
        pub fun_disabled: Option<bool>,
        #[doc = "Whether user installation of apps is disabled."]
        #[serde(rename = "installAppsDisabled", default)]
        pub install_apps_disabled: Option<bool>,
        #[doc = "Whether the user is allowed to enable the \"Unknown Sources\" setting, which allows installation of apps from unknown sources."]
        #[serde(rename = "installUnknownSourcesAllowed", default)]
        pub install_unknown_sources_allowed: Option<bool>,
        #[doc = "Whether the keyguard is disabled."]
        #[serde(rename = "keyguardDisabled", default)]
        pub keyguard_disabled: Option<bool>,
        #[doc = "Disabled keyguard customizations, such as widgets."]
        #[serde(rename = "keyguardDisabledFeatures", default)]
        pub keyguard_disabled_features:
            Option<Vec<crate::schemas::PolicyKeyguardDisabledFeaturesItems>>,
        #[doc = "Whether the kiosk custom launcher is enabled. This replaces the home screen with a launcher that locks down the device to the apps installed via the applications setting. Apps appear on a single page in alphabetical order. The status bar is disabled when this is set."]
        #[serde(rename = "kioskCustomLauncherEnabled", default)]
        pub kiosk_custom_launcher_enabled: Option<bool>,
        #[doc = "The degree of location detection enabled. The user may change the value unless the user is otherwise blocked from accessing device settings."]
        #[serde(rename = "locationMode", default)]
        pub location_mode: Option<crate::schemas::PolicyLocationMode>,
        #[doc = "A message displayed to the user in the device administators settings screen."]
        #[serde(rename = "longSupportMessage", default)]
        pub long_support_message: Option<crate::schemas::UserFacingMessage>,
        #[doc = "Maximum time in milliseconds for user activity until the device locks. A value of 0 means there is no restriction."]
        #[serde(rename = "maximumTimeToLock", default)]
        #[serde(with = "crate::parsed_string")]
        pub maximum_time_to_lock: Option<i64>,
        #[doc = "The minimum allowed Android API level."]
        #[serde(rename = "minimumApiLevel", default)]
        pub minimum_api_level: Option<i32>,
        #[doc = "Whether configuring mobile networks is disabled."]
        #[serde(rename = "mobileNetworksConfigDisabled", default)]
        pub mobile_networks_config_disabled: Option<bool>,
        #[doc = "Whether adding or removing accounts is disabled."]
        #[serde(rename = "modifyAccountsDisabled", default)]
        pub modify_accounts_disabled: Option<bool>,
        #[doc = "Whether the user mounting physical external media is disabled."]
        #[serde(rename = "mountPhysicalMediaDisabled", default)]
        pub mount_physical_media_disabled: Option<bool>,
        #[doc = "The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Whether the network escape hatch is enabled. If a network connection can't be made at boot time, the escape hatch prompts the user to temporarily connect to a network in order to refresh the device policy. After applying policy, the temporary network will be forgotten and the device will continue booting. This prevents being unable to connect to a network if there is no suitable network in the last policy and the device boots into an app in lock task mode, or the user is otherwise unable to reach device settings."]
        #[serde(rename = "networkEscapeHatchEnabled", default)]
        pub network_escape_hatch_enabled: Option<bool>,
        #[doc = "Whether resetting network settings is disabled."]
        #[serde(rename = "networkResetDisabled", default)]
        pub network_reset_disabled: Option<bool>,
        #[doc = "Network configuration for the device. See configure networks for more information."]
        #[serde(rename = "openNetworkConfiguration", default)]
        pub open_network_configuration:
            Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Whether using NFC to beam data from apps is disabled."]
        #[serde(rename = "outgoingBeamDisabled", default)]
        pub outgoing_beam_disabled: Option<bool>,
        #[doc = "Whether outgoing calls are disabled."]
        #[serde(rename = "outgoingCallsDisabled", default)]
        pub outgoing_calls_disabled: Option<bool>,
        #[doc = "Password requirement policies. Different policies can be set for work profile or fully managed devices by setting the password_scope field in the policy."]
        #[serde(rename = "passwordPolicies", default)]
        pub password_policies: Option<Vec<crate::schemas::PasswordRequirements>>,
        #[doc = "Password requirements. DEPRECATED - Use password_policies"]
        #[serde(rename = "passwordRequirements", default)]
        pub password_requirements: Option<crate::schemas::PasswordRequirements>,
        #[doc = "Explicit permission or group grants or denials for all apps. These values override the default_permission_policy."]
        #[serde(rename = "permissionGrants", default)]
        pub permission_grants: Option<Vec<crate::schemas::PermissionGrant>>,
        #[doc = "If present, only the input methods provided by packages in this list are permitted. If this field is present, but the list is empty, then only system input methods are permitted."]
        #[serde(rename = "permittedInputMethods", default)]
        pub permitted_input_methods: Option<crate::schemas::PackageNameList>,
        #[doc = "Default intent handler activities."]
        #[serde(rename = "persistentPreferredActivities", default)]
        pub persistent_preferred_activities:
            Option<Vec<crate::schemas::PersistentPreferredActivity>>,
        #[doc = "This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy."]
        #[serde(rename = "playStoreMode", default)]
        pub play_store_mode: Option<crate::schemas::PolicyPlayStoreMode>,
        #[doc = "Rules that define the behavior when a particular policy can not be applied on device"]
        #[serde(rename = "policyEnforcementRules", default)]
        pub policy_enforcement_rules: Option<Vec<crate::schemas::PolicyEnforcementRule>>,
        #[doc = "Allows showing UI on a device for a user to choose a private key alias if there are no matching rules in ChoosePrivateKeyRules. For devices below Android P, setting this may leave enterprise keys vulnerable."]
        #[serde(rename = "privateKeySelectionEnabled", default)]
        pub private_key_selection_enabled: Option<bool>,
        #[doc = "The network-independent global HTTP proxy. Typically proxies should be configured per-network in open_network_configuration. However for unusual configurations like general internal filtering a global HTTP proxy may be useful. If the proxy is not accessible, network access may break. The global proxy is only a recommendation and some apps may ignore it."]
        #[serde(rename = "recommendedGlobalProxy", default)]
        pub recommended_global_proxy: Option<crate::schemas::ProxyInfo>,
        #[doc = "Whether removing other users is disabled."]
        #[serde(rename = "removeUserDisabled", default)]
        pub remove_user_disabled: Option<bool>,
        #[doc = "Whether rebooting the device into safe boot is disabled."]
        #[serde(rename = "safeBootDisabled", default)]
        pub safe_boot_disabled: Option<bool>,
        #[doc = "Whether screen capture is disabled."]
        #[serde(rename = "screenCaptureDisabled", default)]
        pub screen_capture_disabled: Option<bool>,
        #[doc = "Whether changing the user icon is disabled."]
        #[serde(rename = "setUserIconDisabled", default)]
        pub set_user_icon_disabled: Option<bool>,
        #[doc = "Whether changing the wallpaper is disabled."]
        #[serde(rename = "setWallpaperDisabled", default)]
        pub set_wallpaper_disabled: Option<bool>,
        #[doc = "Actions to take during the setup process."]
        #[serde(rename = "setupActions", default)]
        pub setup_actions: Option<Vec<crate::schemas::SetupAction>>,
        #[doc = "Whether location sharing is disabled."]
        #[serde(rename = "shareLocationDisabled", default)]
        pub share_location_disabled: Option<bool>,
        #[doc = "A message displayed to the user in the settings screen wherever functionality has been disabled by the admin."]
        #[serde(rename = "shortSupportMessage", default)]
        pub short_support_message: Option<crate::schemas::UserFacingMessage>,
        #[doc = "Flag to skip hints on the first use. Enterprise admin can enable the system recommendation for apps to skip their user tutorial and other introductory hints on first start-up."]
        #[serde(rename = "skipFirstUseHintsEnabled", default)]
        pub skip_first_use_hints_enabled: Option<bool>,
        #[doc = "Whether sending and receiving SMS messages is disabled."]
        #[serde(rename = "smsDisabled", default)]
        pub sms_disabled: Option<bool>,
        #[doc = "Whether the status bar is disabled. This disables notifications, quick settings, and other screen overlays that allow escape from full-screen mode. DEPRECATED. To disable the status bar on a kiosk device, use InstallType KIOSK or kioskCustomLauncherEnabled."]
        #[serde(rename = "statusBarDisabled", default)]
        pub status_bar_disabled: Option<bool>,
        #[doc = "Status reporting settings"]
        #[serde(rename = "statusReportingSettings", default)]
        pub status_reporting_settings: Option<crate::schemas::StatusReportingSettings>,
        #[doc = "The battery plugged in modes for which the device stays on. When using this setting, it is recommended to clear maximum_time_to_lock so that the device doesn't lock itself while it stays on."]
        #[serde(rename = "stayOnPluggedModes", default)]
        pub stay_on_plugged_modes: Option<Vec<crate::schemas::PolicyStayOnPluggedModesItems>>,
        #[doc = "The system update policy, which controls how OS updates are applied. If the update type is WINDOWED, the update window will automatically apply to Play app updates as well."]
        #[serde(rename = "systemUpdate", default)]
        pub system_update: Option<crate::schemas::SystemUpdate>,
        #[doc = "Whether configuring tethering and portable hotspots is disabled."]
        #[serde(rename = "tetheringConfigDisabled", default)]
        pub tethering_config_disabled: Option<bool>,
        #[doc = "Whether user uninstallation of applications is disabled."]
        #[serde(rename = "uninstallAppsDisabled", default)]
        pub uninstall_apps_disabled: Option<bool>,
        #[doc = "Whether the microphone is muted and adjusting microphone volume is disabled."]
        #[serde(rename = "unmuteMicrophoneDisabled", default)]
        pub unmute_microphone_disabled: Option<bool>,
        #[doc = "Whether transferring files over USB is disabled."]
        #[serde(rename = "usbFileTransferDisabled", default)]
        pub usb_file_transfer_disabled: Option<bool>,
        #[doc = "Whether USB storage is enabled. Deprecated."]
        #[serde(rename = "usbMassStorageEnabled", default)]
        pub usb_mass_storage_enabled: Option<bool>,
        #[doc = "The version of the policy. This is a read-only field. The version is incremented each time the policy is updated."]
        #[serde(rename = "version", default)]
        #[serde(with = "crate::parsed_string")]
        pub version: Option<i64>,
        #[doc = "Whether configuring VPN is disabled."]
        #[serde(rename = "vpnConfigDisabled", default)]
        pub vpn_config_disabled: Option<bool>,
        #[doc = "Whether configuring Wi-Fi access points is disabled."]
        #[serde(rename = "wifiConfigDisabled", default)]
        pub wifi_config_disabled: Option<bool>,
        #[doc = "DEPRECATED - Use wifi_config_disabled."]
        #[serde(rename = "wifiConfigsLockdownEnabled", default)]
        pub wifi_configs_lockdown_enabled: Option<bool>,
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
    pub struct PolicyEnforcementRule {
        #[doc = "An action to block access to apps and data on a fully managed device or in a work profile. This action also triggers a user-facing notification with information (where possible) on how to correct the compliance issue. Note: wipeAction must also be specified."]
        #[serde(rename = "blockAction", default)]
        pub block_action: Option<crate::schemas::BlockAction>,
        #[doc = "The top-level policy to enforce. For example, applications or passwordPolicies."]
        #[serde(rename = "settingName", default)]
        pub setting_name: Option<String>,
        #[doc = "An action to reset a fully managed device or delete a work profile. Note: blockAction must also be specified."]
        #[serde(rename = "wipeAction", default)]
        pub wipe_action: Option<crate::schemas::WipeAction>,
    }
    impl ::field_selector::FieldSelector for PolicyEnforcementRule {
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
    pub enum PostureDetailSecurityRisk {
        #[doc = "Unspecified. Cannot determine the risk detail."]
        SecurityRiskUnspecified,
        #[doc = "SafetyNet detects that the device uses an unknown OS (basicIntegrity check passes while ctsProfileMatch fails)."]
        UnknownOs,
        #[doc = "SafetyNet detects that the device uses a compromised OS (basicIntegrity check fails)."]
        CompromisedOs,
    }
    impl PostureDetailSecurityRisk {
        pub fn as_str(self) -> &'static str {
            match self {
                PostureDetailSecurityRisk::SecurityRiskUnspecified => "SECURITY_RISK_UNSPECIFIED",
                PostureDetailSecurityRisk::UnknownOs => "UNKNOWN_OS",
                PostureDetailSecurityRisk::CompromisedOs => "COMPROMISED_OS",
            }
        }
    }
    impl ::std::fmt::Display for PostureDetailSecurityRisk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PostureDetailSecurityRisk {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PostureDetailSecurityRisk {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SECURITY_RISK_UNSPECIFIED" => PostureDetailSecurityRisk::SecurityRiskUnspecified,
                "UNKNOWN_OS" => PostureDetailSecurityRisk::UnknownOs,
                "COMPROMISED_OS" => PostureDetailSecurityRisk::CompromisedOs,
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
    pub struct PostureDetail {
        #[doc = "Corresponding pieces of advice to mitigate the security risk."]
        #[serde(rename = "advice", default)]
        pub advice: Option<Vec<crate::schemas::UserFacingMessage>>,
        #[doc = "The risk that makes the device not in the most secure state."]
        #[serde(rename = "securityRisk", default)]
        pub security_risk: Option<crate::schemas::PostureDetailSecurityRisk>,
    }
    impl ::field_selector::FieldSelector for PostureDetail {
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
    pub enum PowerManagementEventEventType {
        #[doc = "Unspecified. No events have this type."]
        PowerManagementEventTypeUnspecified,
        #[doc = "Battery level was measured."]
        BatteryLevelCollected,
        #[doc = "The device started charging."]
        PowerConnected,
        #[doc = "The device stopped charging."]
        PowerDisconnected,
        #[doc = "The device entered low-power mode."]
        BatteryLow,
        #[doc = "The device exited low-power mode."]
        BatteryOkay,
        #[doc = "The device booted."]
        BootCompleted,
        #[doc = "The device shut down."]
        Shutdown,
    }
    impl PowerManagementEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                PowerManagementEventEventType::PowerManagementEventTypeUnspecified => {
                    "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED"
                }
                PowerManagementEventEventType::BatteryLevelCollected => "BATTERY_LEVEL_COLLECTED",
                PowerManagementEventEventType::PowerConnected => "POWER_CONNECTED",
                PowerManagementEventEventType::PowerDisconnected => "POWER_DISCONNECTED",
                PowerManagementEventEventType::BatteryLow => "BATTERY_LOW",
                PowerManagementEventEventType::BatteryOkay => "BATTERY_OKAY",
                PowerManagementEventEventType::BootCompleted => "BOOT_COMPLETED",
                PowerManagementEventEventType::Shutdown => "SHUTDOWN",
            }
        }
    }
    impl ::std::fmt::Display for PowerManagementEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PowerManagementEventEventType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PowerManagementEventEventType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED" => {
                    PowerManagementEventEventType::PowerManagementEventTypeUnspecified
                }
                "BATTERY_LEVEL_COLLECTED" => PowerManagementEventEventType::BatteryLevelCollected,
                "POWER_CONNECTED" => PowerManagementEventEventType::PowerConnected,
                "POWER_DISCONNECTED" => PowerManagementEventEventType::PowerDisconnected,
                "BATTERY_LOW" => PowerManagementEventEventType::BatteryLow,
                "BATTERY_OKAY" => PowerManagementEventEventType::BatteryOkay,
                "BOOT_COMPLETED" => PowerManagementEventEventType::BootCompleted,
                "SHUTDOWN" => PowerManagementEventEventType::Shutdown,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PowerManagementEvent {
        #[doc = "For BATTERY_LEVEL_COLLECTED events, the battery level as a percentage."]
        #[serde(rename = "batteryLevel", default)]
        pub battery_level: Option<f32>,
        #[doc = "The creation time of the event."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Event type."]
        #[serde(rename = "eventType", default)]
        pub event_type: Option<crate::schemas::PowerManagementEventEventType>,
    }
    impl ::field_selector::FieldSelector for PowerManagementEvent {
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
    pub struct ProxyInfo {
        #[doc = "For a direct proxy, the hosts for which the proxy is bypassed. The host names may contain wildcards such as *.example.com."]
        #[serde(rename = "excludedHosts", default)]
        pub excluded_hosts: Option<Vec<String>>,
        #[doc = "The host of the direct proxy."]
        #[serde(rename = "host", default)]
        pub host: Option<String>,
        #[doc = "The URI of the PAC script used to configure the proxy."]
        #[serde(rename = "pacUri", default)]
        pub pac_uri: Option<String>,
        #[doc = "The port of the direct proxy."]
        #[serde(rename = "port", default)]
        pub port: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ProxyInfo {
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
    pub enum SecurityPostureDevicePosture {
        #[doc = "Unspecified. It is unable to determine the correct device posture because of insufficient data (for example, in the case of SafetyNet outage, there is no SafetyNet result). There is no posture detail for this posture value."]
        PostureUnspecified,
        #[doc = "The device is in the most secure state (both SafetyNet's ctsProfileMatch check and basicIntegrity check pass)."]
        Secure,
        #[doc = "The device is at risk (both SafetyNet's ctsProfileMatch check and basicIntegrity check pass)."]
        AtRisk,
        #[doc = "The device is potentially compromised (either SafetyNet's ctsProfileMatch check or basicIntegrity check fails)."]
        PotentiallyCompromised,
    }
    impl SecurityPostureDevicePosture {
        pub fn as_str(self) -> &'static str {
            match self {
                SecurityPostureDevicePosture::PostureUnspecified => "POSTURE_UNSPECIFIED",
                SecurityPostureDevicePosture::Secure => "SECURE",
                SecurityPostureDevicePosture::AtRisk => "AT_RISK",
                SecurityPostureDevicePosture::PotentiallyCompromised => "POTENTIALLY_COMPROMISED",
            }
        }
    }
    impl ::std::fmt::Display for SecurityPostureDevicePosture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SecurityPostureDevicePosture {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SecurityPostureDevicePosture {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "POSTURE_UNSPECIFIED" => SecurityPostureDevicePosture::PostureUnspecified,
                "SECURE" => SecurityPostureDevicePosture::Secure,
                "AT_RISK" => SecurityPostureDevicePosture::AtRisk,
                "POTENTIALLY_COMPROMISED" => SecurityPostureDevicePosture::PotentiallyCompromised,
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
    pub struct SecurityPosture {
        #[doc = "Device's security posture value."]
        #[serde(rename = "devicePosture", default)]
        pub device_posture: Option<crate::schemas::SecurityPostureDevicePosture>,
        #[doc = "Details that provide further information if the device is not in the most secure state."]
        #[serde(rename = "postureDetails", default)]
        pub posture_details: Option<Vec<crate::schemas::PostureDetail>>,
    }
    impl ::field_selector::FieldSelector for SecurityPosture {
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
    pub struct SetupAction {
        #[doc = "Description of this action."]
        #[serde(rename = "description", default)]
        pub description: Option<crate::schemas::UserFacingMessage>,
        #[doc = "An action to launch an app."]
        #[serde(rename = "launchApp", default)]
        pub launch_app: Option<crate::schemas::LaunchAppAction>,
        #[doc = "Title of this action."]
        #[serde(rename = "title", default)]
        pub title: Option<crate::schemas::UserFacingMessage>,
    }
    impl ::field_selector::FieldSelector for SetupAction {
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
    pub struct SigninDetail {
        #[doc = "A JSON string whose UTF-8 representation can be used to generate a QR code to enroll a device with this enrollment token. To enroll a device using NFC, the NFC record must contain a serialized java.util.Properties representation of the properties in the JSON. This is a read-only field generated by the server."]
        #[serde(rename = "qrCode", default)]
        pub qr_code: Option<String>,
        #[doc = "An enterprise wide enrollment token used to trigger custom sign-in flow. This is a read-only field generated by the server."]
        #[serde(rename = "signinEnrollmentToken", default)]
        pub signin_enrollment_token: Option<String>,
        #[doc = "Sign-in URL for authentication when device is provisioned with a sign-in enrollment token. The sign-in endpoint should finish authentication flow with a URL in the form of https://enterprise.google.com/android/enroll?et=<token> for a successful login, or https://enterprise.google.com/android/enroll/invalid for a failed login."]
        #[serde(rename = "signinUrl", default)]
        pub signin_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for SigninDetail {
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
    pub struct SignupUrl {
        #[doc = "The name of the resource. Use this value in the signupUrl field when calling enterprises.create to complete the enterprise signup flow."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "A URL where an enterprise admin can register their enterprise. The page can't be rendered in an iframe."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for SignupUrl {
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
    pub struct SoftwareInfo {
        #[doc = "Android build ID string meant for displaying to the user. For example, shamu-userdebug 6.0.1 MOB30I 2756745 dev-keys."]
        #[serde(rename = "androidBuildNumber", default)]
        pub android_build_number: Option<String>,
        #[doc = "Build time."]
        #[serde(rename = "androidBuildTime", default)]
        pub android_build_time: Option<String>,
        #[doc = "The Android Device Policy app version code."]
        #[serde(rename = "androidDevicePolicyVersionCode", default)]
        pub android_device_policy_version_code: Option<i32>,
        #[doc = "The Android Device Policy app version as displayed to the user."]
        #[serde(rename = "androidDevicePolicyVersionName", default)]
        pub android_device_policy_version_name: Option<String>,
        #[doc = "The user-visible Android version string. For example, 6.0.1."]
        #[serde(rename = "androidVersion", default)]
        pub android_version: Option<String>,
        #[doc = "The system bootloader version number, e.g. 0.6.7."]
        #[serde(rename = "bootloaderVersion", default)]
        pub bootloader_version: Option<String>,
        #[doc = "SHA-256 hash of android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the system package, which can be used to verify that the system build hasn't been modified."]
        #[serde(rename = "deviceBuildSignature", default)]
        pub device_build_signature: Option<String>,
        #[doc = "Kernel version, for example, 2.6.32.9-g103d848."]
        #[serde(rename = "deviceKernelVersion", default)]
        pub device_kernel_version: Option<String>,
        #[doc = "An IETF BCP 47 language code for the primary locale on the device."]
        #[serde(rename = "primaryLanguageCode", default)]
        pub primary_language_code: Option<String>,
        #[doc = "Security patch level, e.g. 2016-05-01."]
        #[serde(rename = "securityPatchLevel", default)]
        pub security_patch_level: Option<String>,
    }
    impl ::field_selector::FieldSelector for SoftwareInfo {
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
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
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
    pub struct StatusReportingSettings {
        #[doc = "Application reporting settings. Only applicable if application_reports_enabled is true."]
        #[serde(rename = "applicationReportingSettings", default)]
        pub application_reporting_settings: Option<crate::schemas::ApplicationReportingSettings>,
        #[doc = "Whether app reports are enabled."]
        #[serde(rename = "applicationReportsEnabled", default)]
        pub application_reports_enabled: Option<bool>,
        #[doc = "Whether device settings reporting is enabled."]
        #[serde(rename = "deviceSettingsEnabled", default)]
        pub device_settings_enabled: Option<bool>,
        #[doc = "Whether displays reporting is enabled."]
        #[serde(rename = "displayInfoEnabled", default)]
        pub display_info_enabled: Option<bool>,
        #[doc = "Whether hardware status reporting is enabled."]
        #[serde(rename = "hardwareStatusEnabled", default)]
        pub hardware_status_enabled: Option<bool>,
        #[doc = "Whether memory reporting is enabled."]
        #[serde(rename = "memoryInfoEnabled", default)]
        pub memory_info_enabled: Option<bool>,
        #[doc = "Whether network info reporting is enabled."]
        #[serde(rename = "networkInfoEnabled", default)]
        pub network_info_enabled: Option<bool>,
        #[doc = "Whether power management event reporting is enabled."]
        #[serde(rename = "powerManagementEventsEnabled", default)]
        pub power_management_events_enabled: Option<bool>,
        #[doc = "Whether software info reporting is enabled."]
        #[serde(rename = "softwareInfoEnabled", default)]
        pub software_info_enabled: Option<bool>,
        #[doc = "Whether system properties reporting is enabled."]
        #[serde(rename = "systemPropertiesEnabled", default)]
        pub system_properties_enabled: Option<bool>,
    }
    impl ::field_selector::FieldSelector for StatusReportingSettings {
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
    pub enum SystemUpdateType {
        #[doc = "Follow the default update behavior for the device, which typically requires the user to accept system updates."]
        SystemUpdateTypeUnspecified,
        #[doc = "Install automatically as soon as an update is available."]
        Automatic,
        #[doc = "Install automatically within a daily maintenance window. This also configures Play apps to be updated within the window. This is strongly recommended for kiosk devices because this is the only way apps persistently pinned to the foreground can be updated by Play."]
        Windowed,
        #[doc = "Postpone automatic install up to a maximum of 30 days."]
        Postpone,
    }
    impl SystemUpdateType {
        pub fn as_str(self) -> &'static str {
            match self {
                SystemUpdateType::SystemUpdateTypeUnspecified => "SYSTEM_UPDATE_TYPE_UNSPECIFIED",
                SystemUpdateType::Automatic => "AUTOMATIC",
                SystemUpdateType::Windowed => "WINDOWED",
                SystemUpdateType::Postpone => "POSTPONE",
            }
        }
    }
    impl ::std::fmt::Display for SystemUpdateType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SystemUpdateType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SystemUpdateType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYSTEM_UPDATE_TYPE_UNSPECIFIED" => SystemUpdateType::SystemUpdateTypeUnspecified,
                "AUTOMATIC" => SystemUpdateType::Automatic,
                "WINDOWED" => SystemUpdateType::Windowed,
                "POSTPONE" => SystemUpdateType::Postpone,
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
    pub struct SystemUpdate {
        #[doc = "If the type is WINDOWED, the end of the maintenance window, measured as the number of minutes after midnight in device's local time. This value must be between 0 and 1439, inclusive. If this value is less than start_minutes, then the maintenance window spans midnight. If the maintenance window specified is smaller than 30 minutes, the actual window is extended to 30 minutes beyond the start time."]
        #[serde(rename = "endMinutes", default)]
        pub end_minutes: Option<i32>,
        #[doc = "The type of system update to configure."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::SystemUpdateType>,
        #[doc = "If the type is WINDOWED, the start of the maintenance window, measured as the number of minutes after midnight in the device's local time. This value must be between 0 and 1439, inclusive."]
        #[serde(rename = "startMinutes", default)]
        pub start_minutes: Option<i32>,
    }
    impl ::field_selector::FieldSelector for SystemUpdate {
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
    pub struct TermsAndConditions {
        #[doc = "A well-formatted HTML string. It will be parsed on the client with android.text.Html#fromHtml."]
        #[serde(rename = "content", default)]
        pub content: Option<crate::schemas::UserFacingMessage>,
        #[doc = "A short header which appears above the HTML content."]
        #[serde(rename = "header", default)]
        pub header: Option<crate::schemas::UserFacingMessage>,
    }
    impl ::field_selector::FieldSelector for TermsAndConditions {
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
    pub struct User {
        #[doc = "A unique identifier you create for this user, such as user342 or asset#44418. This field must be set when the user is created and can't be updated. This field must not contain personally identifiable information (PII). This identifier must be 1024 characters or less; otherwise, the update policy request will fail."]
        #[serde(rename = "accountIdentifier", default)]
        pub account_identifier: Option<String>,
    }
    impl ::field_selector::FieldSelector for User {
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
    pub struct UserFacingMessage {
        #[doc = "The default message displayed if no localized message is specified or the user's locale doesn't match with any of the localized messages. A default message must be provided if any localized messages are provided."]
        #[serde(rename = "defaultMessage", default)]
        pub default_message: Option<String>,
        #[doc = "A map containing <locale, message> pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr."]
        #[serde(rename = "localizedMessages", default)]
        pub localized_messages: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for UserFacingMessage {
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
    pub enum WebAppDisplayMode {
        #[doc = "Not used."]
        DisplayModeUnspecified,
        #[doc = "Opens the web app with a minimal set of browser UI elements for controlling navigation and viewing the page URL."]
        MinimalUi,
        #[doc = "Opens the web app to look and feel like a standalone native application. The browser UI elements and page URL are not visible, however the system status bar and back button are visible."]
        Standalone,
        #[doc = "Opens the web app in full screen without any visible controls. The browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area."]
        FullScreen,
    }
    impl WebAppDisplayMode {
        pub fn as_str(self) -> &'static str {
            match self {
                WebAppDisplayMode::DisplayModeUnspecified => "DISPLAY_MODE_UNSPECIFIED",
                WebAppDisplayMode::MinimalUi => "MINIMAL_UI",
                WebAppDisplayMode::Standalone => "STANDALONE",
                WebAppDisplayMode::FullScreen => "FULL_SCREEN",
            }
        }
    }
    impl ::std::fmt::Display for WebAppDisplayMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WebAppDisplayMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WebAppDisplayMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISPLAY_MODE_UNSPECIFIED" => WebAppDisplayMode::DisplayModeUnspecified,
                "MINIMAL_UI" => WebAppDisplayMode::MinimalUi,
                "STANDALONE" => WebAppDisplayMode::Standalone,
                "FULL_SCREEN" => WebAppDisplayMode::FullScreen,
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
    pub struct WebApp {
        #[doc = "The display mode of the web app."]
        #[serde(rename = "displayMode", default)]
        pub display_mode: Option<crate::schemas::WebAppDisplayMode>,
        #[doc = "A list of icons for the web app. Must have at least one element."]
        #[serde(rename = "icons", default)]
        pub icons: Option<Vec<crate::schemas::WebAppIcon>>,
        #[doc = "The name of the web app, which is generated by the server during creation in the form enterprises/{enterpriseId}/webApps/{packageName}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The start URL, i.e. the URL that should load when the user opens the application."]
        #[serde(rename = "startUrl", default)]
        pub start_url: Option<String>,
        #[doc = "The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon)."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "The current version of the app.Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date."]
        #[serde(rename = "versionCode", default)]
        #[serde(with = "crate::parsed_string")]
        pub version_code: Option<i64>,
    }
    impl ::field_selector::FieldSelector for WebApp {
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
    pub struct WebAppIcon {
        #[doc = "The actual bytes of the image in a base64url encoded string (c.f. RFC4648, section 5 \"Base 64 Encoding with URL and Filename Safe Alphabet\"). <ul> <li>The image type can be png or jpg. <li>The image should ideally be square. <li>The image should ideally have a size of 512x512. </ul>"]
        #[serde(rename = "imageData", default)]
        pub image_data: Option<String>,
    }
    impl ::field_selector::FieldSelector for WebAppIcon {
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
    pub enum WebTokenPermissionsItems {}
    impl WebTokenPermissionsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for WebTokenPermissionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WebTokenPermissionsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WebTokenPermissionsItems {
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
    pub struct WebToken {
        #[doc = "The name of the web token, which is generated by the server during creation in the form enterprises/{enterpriseId}/webTokens/{webTokenId}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The URL of the parent frame hosting the iframe with the embedded UI. To prevent XSS, the iframe may not be hosted at other URLs. The URL must use the https scheme."]
        #[serde(rename = "parentFrameUrl", default)]
        pub parent_frame_url: Option<String>,
        #[doc = "Permissions available to an admin in the embedded UI. An admin must have all of these permissions in order to view the UI. This field is deprecated."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<crate::schemas::WebTokenPermissionsItems>>,
        #[doc = "The token value which is used in the hosting page to generate the iframe with the embedded UI. This is a read-only field generated by the server."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for WebToken {
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
    pub struct WipeAction {
        #[doc = "Whether the factory-reset protection data is preserved on the device. This setting doesn\u{2019}t apply to work profiles."]
        #[serde(rename = "preserveFrp", default)]
        pub preserve_frp: Option<bool>,
        #[doc = "Number of days the policy is non-compliant before the device or work profile is wiped. wipeAfterDays must be greater than blockAfterDays."]
        #[serde(rename = "wipeAfterDays", default)]
        pub wipe_after_days: Option<i32>,
    }
    impl ::field_selector::FieldSelector for WipeAction {
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
    #[doc = "Actions that can be performed on the enterprises resource"]
    pub fn enterprises(&self) -> crate::enterprises::EnterprisesActions<A> {
        crate::enterprises::EnterprisesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the signup_urls resource"]
    pub fn signup_urls(&self) -> crate::signup_urls::SignupUrlsActions<A> {
        crate::signup_urls::SignupUrlsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod enterprises {
    pub mod params {}
    pub struct EnterprisesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> EnterprisesActions<'a, A> {
        #[doc = "Creates an enterprise. This is the last step in the enterprise signup flow."]
        pub fn create(&self, request: crate::schemas::Enterprise) -> CreateRequestBuilder<A> {
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
                enterprise_token: None,
                project_id: None,
                signup_url_name: None,
            }
        }
        #[doc = "Gets an enterprise."]
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
        #[doc = "Updates an enterprise."]
        pub fn patch(
            &self,
            request: crate::schemas::Enterprise,
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
        #[doc = "Actions that can be performed on the applications resource"]
        pub fn applications(&self) -> applications::ApplicationsActions<A> {
            applications::ApplicationsActions
        }
        #[doc = "Actions that can be performed on the devices resource"]
        pub fn devices(&self) -> devices::DevicesActions<A> {
            devices::DevicesActions
        }
        #[doc = "Actions that can be performed on the enrollment_tokens resource"]
        pub fn enrollment_tokens(&self) -> enrollment_tokens::EnrollmentTokensActions<A> {
            enrollment_tokens::EnrollmentTokensActions
        }
        #[doc = "Actions that can be performed on the policies resource"]
        pub fn policies(&self) -> policies::PoliciesActions<A> {
            policies::PoliciesActions
        }
        #[doc = "Actions that can be performed on the web_apps resource"]
        pub fn web_apps(&self) -> web_apps::WebAppsActions<A> {
            web_apps::WebAppsActions
        }
        #[doc = "Actions that can be performed on the web_tokens resource"]
        pub fn web_tokens(&self) -> web_tokens::WebTokensActions<A> {
            web_tokens::WebTokensActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Enterprise,
        enterprise_token: Option<String>,
        project_id: Option<String>,
        signup_url_name: Option<String>,
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
        #[doc = "The enterprise token appended to the callback URL."]
        pub fn enterprise_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.enterprise_token = Some(value.into());
            self
        }
        #[doc = "The ID of the Google Cloud Platform project which will own the enterprise."]
        pub fn project_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.project_id = Some(value.into());
            self
        }
        #[doc = "The name of the SignupUrl used to sign up for the enterprise."]
        pub fn signup_url_name(&mut self, value: impl Into<String>) -> &mut Self {
            self.signup_url_name = Some(value.into());
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
        ) -> Result<crate::schemas::Enterprise, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://androidmanagement.googleapis.com/".to_owned();
            output.push_str("v1/enterprises");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("enterpriseToken", &self.enterprise_token)]);
            let req = req.query(&[("projectId", &self.project_id)]);
            let req = req.query(&[("signupUrlName", &self.signup_url_name)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
        ) -> Result<crate::schemas::Enterprise, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://androidmanagement.googleapis.com/".to_owned();
            output.push_str("v1/");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
        request: crate::schemas::Enterprise,
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
        #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
        ) -> Result<crate::schemas::Enterprise, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://androidmanagement.googleapis.com/".to_owned();
            output.push_str("v1/");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    pub mod applications {
        pub mod params {}
        pub struct ApplicationsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ApplicationsActions<'a, A> {
            #[doc = "Gets info about an application."]
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
                    language_code: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
            language_code: Option<String>,
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
            #[doc = "The preferred language for localized application info, as a BCP47 tag (e.g. \"en-US\", \"de\"). If not specified the default language of the application will be used."]
            pub fn language_code(&mut self, value: impl Into<String>) -> &mut Self {
                self.language_code = Some(value.into());
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
            ) -> Result<crate::schemas::Application, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("languageCode", &self.language_code)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod devices {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum DeleteWipeDataFlags {}
            impl DeleteWipeDataFlags {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for DeleteWipeDataFlags {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for DeleteWipeDataFlags {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for DeleteWipeDataFlags {
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
        pub struct DevicesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DevicesActions<'a, A> {
            #[doc = "Deletes a device. This operation wipes the device."]
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
                    wipe_data_flags: None,
                }
            }
            #[doc = "Gets a device."]
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
            #[doc = "Issues a command to a device. The Operation resource returned contains a Command in its metadata field. Use the get operation method to get the status of the command."]
            pub fn issue_command(
                &self,
                request: crate::schemas::Command,
                name: impl Into<String>,
            ) -> IssueCommandRequestBuilder<A> {
                IssueCommandRequestBuilder {
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
            #[doc = "Lists devices for a given enterprise."]
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
            #[doc = "Updates a device."]
            pub fn patch(
                &self,
                request: crate::schemas::Device,
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
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(&self) -> operations::OperationsActions<A> {
                operations::OperationsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
            wipe_data_flags: Option<crate::devices::params::DeleteWipeDataFlags>,
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
            #[doc = "Optional flags that control the device wiping behavior."]
            pub fn wipe_data_flags(
                &mut self,
                value: crate::devices::params::DeleteWipeDataFlags,
            ) -> &mut Self {
                self.wipe_data_flags = Some(value);
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("wipeDataFlags", &self.wipe_data_flags)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            ) -> Result<crate::schemas::Device, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct IssueCommandRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Command,
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
        impl<'a, A: yup_oauth2::GetToken> IssueCommandRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.name);
                output.push_str(":issueCommand");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results returned by the server."]
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
            pub fn iter_devices<T>(
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
                            #[serde(rename = "devices")]
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
            ) -> Result<crate::schemas::ListDevicesResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/devices");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            request: crate::schemas::Device,
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
            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
            ) -> Result<crate::schemas::Device, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> OperationsActions<'a, A> {
                #[doc = "Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED."]
                pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder<A> {
                    CancelRequestBuilder {
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
                #[doc = "Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED."]
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
                #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
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
                #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
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
            pub struct CancelRequestBuilder<'a, A> {
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
            impl<'a, A: yup_oauth2::GetToken> CancelRequestBuilder<'a, A> {
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
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    output.push_str(&self.name);
                    output.push_str(":cancel");
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
                            "https://www.googleapis.com/auth/androidmanagement",
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
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
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
                            "https://www.googleapis.com/auth/androidmanagement",
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
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
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
                            "https://www.googleapis.com/auth/androidmanagement",
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
                    let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    output.push_str(&self.name);
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
                            "https://www.googleapis.com/auth/androidmanagement",
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
    pub mod enrollment_tokens {
        pub mod params {}
        pub struct EnrollmentTokensActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> EnrollmentTokensActions<'a, A> {
            #[doc = "Creates an enrollment token for a given enterprise."]
            pub fn create(
                &self,
                request: crate::schemas::EnrollmentToken,
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
            #[doc = "Deletes an enrollment token. This operation invalidates the token, preventing its future use."]
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
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::EnrollmentToken,
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
            ) -> Result<crate::schemas::EnrollmentToken, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/enrollmentTokens");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod policies {
        pub mod params {}
        pub struct PoliciesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PoliciesActions<'a, A> {
            #[doc = "Deletes a policy. This operation is only permitted if no devices are currently referencing the policy."]
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
            #[doc = "Gets a policy."]
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
            #[doc = "Lists policies for a given enterprise."]
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
            #[doc = "Updates or creates a policy."]
            pub fn patch(
                &self,
                request: crate::schemas::Policy,
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results returned by the server."]
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
            pub fn iter_policies<T>(
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
                            #[serde(rename = "policies")]
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
            ) -> Result<crate::schemas::ListPoliciesResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/policies");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            request: crate::schemas::Policy,
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
            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod web_apps {
        pub mod params {}
        pub struct WebAppsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> WebAppsActions<'a, A> {
            #[doc = "Creates a web app."]
            pub fn create(
                &self,
                request: crate::schemas::WebApp,
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
            #[doc = "Deletes a web app."]
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
            #[doc = "Gets a web app."]
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
            #[doc = "Lists web apps for a given enterprise."]
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
            #[doc = "Updates a web app."]
            pub fn patch(
                &self,
                request: crate::schemas::WebApp,
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
            request: crate::schemas::WebApp,
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
            ) -> Result<crate::schemas::WebApp, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/webApps");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            ) -> Result<crate::schemas::WebApp, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            #[doc = "The requested page size. The actual page size may be fixed to a min or max value."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results returned by the server."]
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
            pub fn iter_web_apps<T>(
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
                            #[serde(rename = "webApps")]
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
            ) -> Result<crate::schemas::ListWebAppsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/webApps");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
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
            request: crate::schemas::WebApp,
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
            #[doc = "The field mask indicating the fields to update. If not set, all modifiable fields will be modified."]
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
            ) -> Result<crate::schemas::WebApp, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod web_tokens {
        pub mod params {}
        pub struct WebTokensActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> WebTokensActions<'a, A> {
            #[doc = "Creates a web token to access an embeddable managed Google Play web UI for a given enterprise."]
            pub fn create(
                &self,
                request: crate::schemas::WebToken,
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
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::WebToken,
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
            ) -> Result<crate::schemas::WebToken, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://androidmanagement.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/webTokens");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
}
pub mod signup_urls {
    pub mod params {}
    pub struct SignupUrlsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> SignupUrlsActions<'a, A> {
        #[doc = "Creates an enterprise signup URL."]
        pub fn create(&self) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                callback_url: None,
                project_id: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        callback_url: Option<String>,
        project_id: Option<String>,
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
        #[doc = "The callback URL that the admin will be redirected to after successfully creating an enterprise. Before redirecting there the system will add a query parameter to this URL named enterpriseToken which will contain an opaque token to be used for the create enterprise request. The URL will be parsed then reformatted in order to add the enterpriseToken parameter, so there may be some minor formatting changes."]
        pub fn callback_url(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback_url = Some(value.into());
            self
        }
        #[doc = "The ID of the Google Cloud Platform project which will own the enterprise."]
        pub fn project_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.project_id = Some(value.into());
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
        ) -> Result<crate::schemas::SignupUrl, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://androidmanagement.googleapis.com/".to_owned();
            output.push_str("v1/signupUrls");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("callbackUrl", &self.callback_url)]);
            let req = req.query(&[("projectId", &self.project_id)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/androidmanagement"])
                    .unwrap()
                    .access_token,
            );
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
