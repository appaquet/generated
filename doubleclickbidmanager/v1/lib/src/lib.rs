pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DownloadLineItemsRequestFileSpec {
        Ewf,
    }
    impl DownloadLineItemsRequestFileSpec {
        pub fn as_str(self) -> &'static str {
            match self {
                DownloadLineItemsRequestFileSpec::Ewf => "EWF",
            }
        }
    }
    impl ::std::fmt::Display for DownloadLineItemsRequestFileSpec {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DownloadLineItemsRequestFileSpec {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DownloadLineItemsRequestFileSpec {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EWF" => DownloadLineItemsRequestFileSpec::Ewf,
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
    pub enum DownloadLineItemsRequestFilterType {
        AdvertiserId,
        InsertionOrderId,
        LineItemId,
    }
    impl DownloadLineItemsRequestFilterType {
        pub fn as_str(self) -> &'static str {
            match self {
                DownloadLineItemsRequestFilterType::AdvertiserId => "ADVERTISER_ID",
                DownloadLineItemsRequestFilterType::InsertionOrderId => "INSERTION_ORDER_ID",
                DownloadLineItemsRequestFilterType::LineItemId => "LINE_ITEM_ID",
            }
        }
    }
    impl ::std::fmt::Display for DownloadLineItemsRequestFilterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DownloadLineItemsRequestFilterType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DownloadLineItemsRequestFilterType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADVERTISER_ID" => DownloadLineItemsRequestFilterType::AdvertiserId,
                "INSERTION_ORDER_ID" => DownloadLineItemsRequestFilterType::InsertionOrderId,
                "LINE_ITEM_ID" => DownloadLineItemsRequestFilterType::LineItemId,
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
    pub enum DownloadLineItemsRequestFormat {
        Csv,
    }
    impl DownloadLineItemsRequestFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                DownloadLineItemsRequestFormat::Csv => "CSV",
            }
        }
    }
    impl ::std::fmt::Display for DownloadLineItemsRequestFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DownloadLineItemsRequestFormat {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DownloadLineItemsRequestFormat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => DownloadLineItemsRequestFormat::Csv,
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
    pub struct DownloadLineItemsRequest {
        #[doc = "File specification (column names, types, order) in which the line items will be returned. Default to EWF."]
        #[serde(rename = "fileSpec", default)]
        pub file_spec: Option<crate::schemas::DownloadLineItemsRequestFileSpec>,
        #[doc = "Ids of the specified filter type used to filter line items to fetch. If omitted, all the line items will be returned."]
        #[serde(rename = "filterIds", default)]
        pub filter_ids: Option<Vec<i64>>,
        #[doc = "Filter type used to filter line items to fetch."]
        #[serde(rename = "filterType", default)]
        pub filter_type: Option<crate::schemas::DownloadLineItemsRequestFilterType>,
        #[doc = "Format in which the line items will be returned. Default to CSV."]
        #[serde(rename = "format", default)]
        pub format: Option<crate::schemas::DownloadLineItemsRequestFormat>,
    }
    impl ::field_selector::FieldSelector for DownloadLineItemsRequest {
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
    pub struct DownloadLineItemsResponse {
        #[doc = "Retrieved line items in CSV format. For more information about file formats, see  Entity Write File Format."]
        #[serde(rename = "lineItems", default)]
        pub line_items: Option<String>,
    }
    impl ::field_selector::FieldSelector for DownloadLineItemsResponse {
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
    pub enum DownloadRequestFileTypesItems {
        Ad,
        AdGroup,
        Campaign,
        InsertionOrder,
        InventorySource,
        LineItem,
    }
    impl DownloadRequestFileTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                DownloadRequestFileTypesItems::Ad => "AD",
                DownloadRequestFileTypesItems::AdGroup => "AD_GROUP",
                DownloadRequestFileTypesItems::Campaign => "CAMPAIGN",
                DownloadRequestFileTypesItems::InsertionOrder => "INSERTION_ORDER",
                DownloadRequestFileTypesItems::InventorySource => "INVENTORY_SOURCE",
                DownloadRequestFileTypesItems::LineItem => "LINE_ITEM",
            }
        }
    }
    impl ::std::fmt::Display for DownloadRequestFileTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DownloadRequestFileTypesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DownloadRequestFileTypesItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD" => DownloadRequestFileTypesItems::Ad,
                "AD_GROUP" => DownloadRequestFileTypesItems::AdGroup,
                "CAMPAIGN" => DownloadRequestFileTypesItems::Campaign,
                "INSERTION_ORDER" => DownloadRequestFileTypesItems::InsertionOrder,
                "INVENTORY_SOURCE" => DownloadRequestFileTypesItems::InventorySource,
                "LINE_ITEM" => DownloadRequestFileTypesItems::LineItem,
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
    pub enum DownloadRequestFilterType {
        AdvertiserId,
        CampaignId,
        InsertionOrderId,
        InventorySourceId,
        LineItemId,
        PartnerId,
    }
    impl DownloadRequestFilterType {
        pub fn as_str(self) -> &'static str {
            match self {
                DownloadRequestFilterType::AdvertiserId => "ADVERTISER_ID",
                DownloadRequestFilterType::CampaignId => "CAMPAIGN_ID",
                DownloadRequestFilterType::InsertionOrderId => "INSERTION_ORDER_ID",
                DownloadRequestFilterType::InventorySourceId => "INVENTORY_SOURCE_ID",
                DownloadRequestFilterType::LineItemId => "LINE_ITEM_ID",
                DownloadRequestFilterType::PartnerId => "PARTNER_ID",
            }
        }
    }
    impl ::std::fmt::Display for DownloadRequestFilterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DownloadRequestFilterType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DownloadRequestFilterType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADVERTISER_ID" => DownloadRequestFilterType::AdvertiserId,
                "CAMPAIGN_ID" => DownloadRequestFilterType::CampaignId,
                "INSERTION_ORDER_ID" => DownloadRequestFilterType::InsertionOrderId,
                "INVENTORY_SOURCE_ID" => DownloadRequestFilterType::InventorySourceId,
                "LINE_ITEM_ID" => DownloadRequestFilterType::LineItemId,
                "PARTNER_ID" => DownloadRequestFilterType::PartnerId,
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
    pub struct DownloadRequest {
        #[doc = "File types that will be returned. If INVENTORY_SOURCE is requested, no other file types may be requested.\n\nAcceptable values are:  \n- \"AD\" \n- \"AD_GROUP\" \n- \"CAMPAIGN\" \n- \"INSERTION_ORDER\" \n- \"INVENTORY_SOURCE\" \n- \"LINE_ITEM\""]
        #[serde(rename = "fileTypes", default)]
        pub file_types: Option<Vec<crate::schemas::DownloadRequestFileTypesItems>>,
        #[doc = "The IDs of the specified filter type. This is used to filter entities to fetch. At least one ID must be specified."]
        #[serde(rename = "filterIds", default)]
        pub filter_ids: Option<Vec<i64>>,
        #[doc = "Filter type used to filter entities to fetch. PARTNER_ID and INVENTORY_SOURCE_ID may only be used when downloading inventory sources."]
        #[serde(rename = "filterType", default)]
        pub filter_type: Option<crate::schemas::DownloadRequestFilterType>,
        #[doc = "SDF Version (column names, types, order) in which the entities will be returned. Default to 3.1."]
        #[serde(rename = "version", default)]
        pub version: Option<String>,
    }
    impl ::field_selector::FieldSelector for DownloadRequest {
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
    pub struct DownloadResponse {
        #[doc = "Retrieved ad groups in SDF format."]
        #[serde(rename = "adGroups", default)]
        pub ad_groups: Option<String>,
        #[doc = "Retrieved ads in SDF format."]
        #[serde(rename = "ads", default)]
        pub ads: Option<String>,
        #[doc = "Retrieved campaigns in SDF format."]
        #[serde(rename = "campaigns", default)]
        pub campaigns: Option<String>,
        #[doc = "Retrieved insertion orders in SDF format."]
        #[serde(rename = "insertionOrders", default)]
        pub insertion_orders: Option<String>,
        #[serde(rename = "inventorySources", default)]
        pub inventory_sources: Option<String>,
        #[doc = "Retrieved line items in SDF format."]
        #[serde(rename = "lineItems", default)]
        pub line_items: Option<String>,
    }
    impl ::field_selector::FieldSelector for DownloadResponse {
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
    pub enum FilterPairType {
        FilterActiveViewExpectedViewability,
        FilterActivityId,
        FilterAdvertiser,
        FilterAdvertiserCurrency,
        FilterAdvertiserTimezone,
        FilterAdPosition,
        FilterAge,
        FilterAuthorizedSellerState,
        FilterBrandsafeChannelId,
        FilterBrowser,
        FilterBudgetSegmentDescription,
        FilterCampaignDailyFrequency,
        FilterCarrier,
        FilterChannelId,
        FilterCity,
        FilterCompanionCreativeId,
        FilterConversionDelay,
        FilterCountry,
        FilterCreativeAttribute,
        FilterCreativeHeight,
        FilterCreativeId,
        FilterCreativeSize,
        FilterCreativeType,
        FilterCreativeWidth,
        FilterDataProvider,
        FilterDate,
        FilterDayOfWeek,
        FilterDeviceMake,
        FilterDeviceModel,
        FilterDeviceType,
        FilterDfpOrderId,
        FilterDma,
        FilterExchangeId,
        FilterFloodlightActivityId,
        FilterGender,
        FilterInsertionOrder,
        FilterInventoryCommitmentType,
        FilterInventoryDeliveryMethod,
        FilterInventoryFormat,
        FilterInventoryRateType,
        FilterInventorySource,
        FilterInventorySourceExternalId,
        FilterInventorySourceType,
        FilterKeyword,
        FilterLineItem,
        FilterLineItemDailyFrequency,
        FilterLineItemLifetimeFrequency,
        FilterLineItemType,
        FilterMediaPlan,
        FilterMobileDeviceMake,
        FilterMobileDeviceMakeModel,
        FilterMobileDeviceType,
        FilterMobileGeo,
        FilterMonth,
        FilterMraidSupport,
        FilterNielsenAge,
        FilterNielsenCountryCode,
        FilterNielsenDeviceId,
        FilterNielsenGender,
        FilterNotSupported,
        FilterOrderId,
        FilterOs,
        FilterPageCategory,
        FilterPageLayout,
        FilterPartner,
        FilterPartnerCurrency,
        FilterPublicInventory,
        FilterQuarter,
        FilterRegion,
        FilterRegularChannelId,
        FilterSiteId,
        FilterSiteLanguage,
        FilterSkippableSupport,
        FilterTargetedUserList,
        FilterTimeOfDay,
        FilterTrueviewAdGroupAdId,
        FilterTrueviewAdGroupId,
        FilterTrueviewAge,
        FilterTrueviewCategory,
        FilterTrueviewCity,
        FilterTrueviewConversionType,
        FilterTrueviewCountry,
        FilterTrueviewCustomAffinity,
        FilterTrueviewDma,
        FilterTrueviewGender,
        FilterTrueviewIarAge,
        FilterTrueviewIarCategory,
        FilterTrueviewIarCity,
        FilterTrueviewIarCountry,
        FilterTrueviewIarGender,
        FilterTrueviewIarInterest,
        FilterTrueviewIarLanguage,
        FilterTrueviewIarParentalStatus,
        FilterTrueviewIarRegion,
        FilterTrueviewIarRemarketingList,
        FilterTrueviewIarTimeOfDay,
        FilterTrueviewIarYoutubeChannel,
        FilterTrueviewIarYoutubeVideo,
        FilterTrueviewIarZipcode,
        FilterTrueviewInterest,
        FilterTrueviewKeyword,
        FilterTrueviewParentalStatus,
        FilterTrueviewPlacement,
        FilterTrueviewRegion,
        FilterTrueviewRemarketingList,
        FilterTrueviewUrl,
        FilterTrueviewZipcode,
        FilterUnknown,
        FilterUserList,
        FilterUserListFirstParty,
        FilterUserListThirdParty,
        FilterVideoAdPositionInStream,
        FilterVideoCompanionSize,
        FilterVideoCompanionType,
        FilterVideoCreativeDuration,
        FilterVideoCreativeDurationSkippable,
        FilterVideoDurationSeconds,
        FilterVideoDurationSecondsRange,
        FilterVideoFormatSupport,
        FilterVideoInventoryType,
        FilterVideoPlayerSize,
        FilterVideoRatingTier,
        FilterVideoSkippableSupport,
        FilterVideoVpaidSupport,
        FilterWeek,
        FilterYear,
        FilterYoutubeVertical,
        FilterZipCode,
    }
    impl FilterPairType {
        pub fn as_str(self) -> &'static str {
            match self {
                FilterPairType::FilterActiveViewExpectedViewability => {
                    "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY"
                }
                FilterPairType::FilterActivityId => "FILTER_ACTIVITY_ID",
                FilterPairType::FilterAdvertiser => "FILTER_ADVERTISER",
                FilterPairType::FilterAdvertiserCurrency => "FILTER_ADVERTISER_CURRENCY",
                FilterPairType::FilterAdvertiserTimezone => "FILTER_ADVERTISER_TIMEZONE",
                FilterPairType::FilterAdPosition => "FILTER_AD_POSITION",
                FilterPairType::FilterAge => "FILTER_AGE",
                FilterPairType::FilterAuthorizedSellerState => "FILTER_AUTHORIZED_SELLER_STATE",
                FilterPairType::FilterBrandsafeChannelId => "FILTER_BRANDSAFE_CHANNEL_ID",
                FilterPairType::FilterBrowser => "FILTER_BROWSER",
                FilterPairType::FilterBudgetSegmentDescription => {
                    "FILTER_BUDGET_SEGMENT_DESCRIPTION"
                }
                FilterPairType::FilterCampaignDailyFrequency => "FILTER_CAMPAIGN_DAILY_FREQUENCY",
                FilterPairType::FilterCarrier => "FILTER_CARRIER",
                FilterPairType::FilterChannelId => "FILTER_CHANNEL_ID",
                FilterPairType::FilterCity => "FILTER_CITY",
                FilterPairType::FilterCompanionCreativeId => "FILTER_COMPANION_CREATIVE_ID",
                FilterPairType::FilterConversionDelay => "FILTER_CONVERSION_DELAY",
                FilterPairType::FilterCountry => "FILTER_COUNTRY",
                FilterPairType::FilterCreativeAttribute => "FILTER_CREATIVE_ATTRIBUTE",
                FilterPairType::FilterCreativeHeight => "FILTER_CREATIVE_HEIGHT",
                FilterPairType::FilterCreativeId => "FILTER_CREATIVE_ID",
                FilterPairType::FilterCreativeSize => "FILTER_CREATIVE_SIZE",
                FilterPairType::FilterCreativeType => "FILTER_CREATIVE_TYPE",
                FilterPairType::FilterCreativeWidth => "FILTER_CREATIVE_WIDTH",
                FilterPairType::FilterDataProvider => "FILTER_DATA_PROVIDER",
                FilterPairType::FilterDate => "FILTER_DATE",
                FilterPairType::FilterDayOfWeek => "FILTER_DAY_OF_WEEK",
                FilterPairType::FilterDeviceMake => "FILTER_DEVICE_MAKE",
                FilterPairType::FilterDeviceModel => "FILTER_DEVICE_MODEL",
                FilterPairType::FilterDeviceType => "FILTER_DEVICE_TYPE",
                FilterPairType::FilterDfpOrderId => "FILTER_DFP_ORDER_ID",
                FilterPairType::FilterDma => "FILTER_DMA",
                FilterPairType::FilterExchangeId => "FILTER_EXCHANGE_ID",
                FilterPairType::FilterFloodlightActivityId => "FILTER_FLOODLIGHT_ACTIVITY_ID",
                FilterPairType::FilterGender => "FILTER_GENDER",
                FilterPairType::FilterInsertionOrder => "FILTER_INSERTION_ORDER",
                FilterPairType::FilterInventoryCommitmentType => "FILTER_INVENTORY_COMMITMENT_TYPE",
                FilterPairType::FilterInventoryDeliveryMethod => "FILTER_INVENTORY_DELIVERY_METHOD",
                FilterPairType::FilterInventoryFormat => "FILTER_INVENTORY_FORMAT",
                FilterPairType::FilterInventoryRateType => "FILTER_INVENTORY_RATE_TYPE",
                FilterPairType::FilterInventorySource => "FILTER_INVENTORY_SOURCE",
                FilterPairType::FilterInventorySourceExternalId => {
                    "FILTER_INVENTORY_SOURCE_EXTERNAL_ID"
                }
                FilterPairType::FilterInventorySourceType => "FILTER_INVENTORY_SOURCE_TYPE",
                FilterPairType::FilterKeyword => "FILTER_KEYWORD",
                FilterPairType::FilterLineItem => "FILTER_LINE_ITEM",
                FilterPairType::FilterLineItemDailyFrequency => "FILTER_LINE_ITEM_DAILY_FREQUENCY",
                FilterPairType::FilterLineItemLifetimeFrequency => {
                    "FILTER_LINE_ITEM_LIFETIME_FREQUENCY"
                }
                FilterPairType::FilterLineItemType => "FILTER_LINE_ITEM_TYPE",
                FilterPairType::FilterMediaPlan => "FILTER_MEDIA_PLAN",
                FilterPairType::FilterMobileDeviceMake => "FILTER_MOBILE_DEVICE_MAKE",
                FilterPairType::FilterMobileDeviceMakeModel => "FILTER_MOBILE_DEVICE_MAKE_MODEL",
                FilterPairType::FilterMobileDeviceType => "FILTER_MOBILE_DEVICE_TYPE",
                FilterPairType::FilterMobileGeo => "FILTER_MOBILE_GEO",
                FilterPairType::FilterMonth => "FILTER_MONTH",
                FilterPairType::FilterMraidSupport => "FILTER_MRAID_SUPPORT",
                FilterPairType::FilterNielsenAge => "FILTER_NIELSEN_AGE",
                FilterPairType::FilterNielsenCountryCode => "FILTER_NIELSEN_COUNTRY_CODE",
                FilterPairType::FilterNielsenDeviceId => "FILTER_NIELSEN_DEVICE_ID",
                FilterPairType::FilterNielsenGender => "FILTER_NIELSEN_GENDER",
                FilterPairType::FilterNotSupported => "FILTER_NOT_SUPPORTED",
                FilterPairType::FilterOrderId => "FILTER_ORDER_ID",
                FilterPairType::FilterOs => "FILTER_OS",
                FilterPairType::FilterPageCategory => "FILTER_PAGE_CATEGORY",
                FilterPairType::FilterPageLayout => "FILTER_PAGE_LAYOUT",
                FilterPairType::FilterPartner => "FILTER_PARTNER",
                FilterPairType::FilterPartnerCurrency => "FILTER_PARTNER_CURRENCY",
                FilterPairType::FilterPublicInventory => "FILTER_PUBLIC_INVENTORY",
                FilterPairType::FilterQuarter => "FILTER_QUARTER",
                FilterPairType::FilterRegion => "FILTER_REGION",
                FilterPairType::FilterRegularChannelId => "FILTER_REGULAR_CHANNEL_ID",
                FilterPairType::FilterSiteId => "FILTER_SITE_ID",
                FilterPairType::FilterSiteLanguage => "FILTER_SITE_LANGUAGE",
                FilterPairType::FilterSkippableSupport => "FILTER_SKIPPABLE_SUPPORT",
                FilterPairType::FilterTargetedUserList => "FILTER_TARGETED_USER_LIST",
                FilterPairType::FilterTimeOfDay => "FILTER_TIME_OF_DAY",
                FilterPairType::FilterTrueviewAdGroupAdId => "FILTER_TRUEVIEW_AD_GROUP_AD_ID",
                FilterPairType::FilterTrueviewAdGroupId => "FILTER_TRUEVIEW_AD_GROUP_ID",
                FilterPairType::FilterTrueviewAge => "FILTER_TRUEVIEW_AGE",
                FilterPairType::FilterTrueviewCategory => "FILTER_TRUEVIEW_CATEGORY",
                FilterPairType::FilterTrueviewCity => "FILTER_TRUEVIEW_CITY",
                FilterPairType::FilterTrueviewConversionType => "FILTER_TRUEVIEW_CONVERSION_TYPE",
                FilterPairType::FilterTrueviewCountry => "FILTER_TRUEVIEW_COUNTRY",
                FilterPairType::FilterTrueviewCustomAffinity => "FILTER_TRUEVIEW_CUSTOM_AFFINITY",
                FilterPairType::FilterTrueviewDma => "FILTER_TRUEVIEW_DMA",
                FilterPairType::FilterTrueviewGender => "FILTER_TRUEVIEW_GENDER",
                FilterPairType::FilterTrueviewIarAge => "FILTER_TRUEVIEW_IAR_AGE",
                FilterPairType::FilterTrueviewIarCategory => "FILTER_TRUEVIEW_IAR_CATEGORY",
                FilterPairType::FilterTrueviewIarCity => "FILTER_TRUEVIEW_IAR_CITY",
                FilterPairType::FilterTrueviewIarCountry => "FILTER_TRUEVIEW_IAR_COUNTRY",
                FilterPairType::FilterTrueviewIarGender => "FILTER_TRUEVIEW_IAR_GENDER",
                FilterPairType::FilterTrueviewIarInterest => "FILTER_TRUEVIEW_IAR_INTEREST",
                FilterPairType::FilterTrueviewIarLanguage => "FILTER_TRUEVIEW_IAR_LANGUAGE",
                FilterPairType::FilterTrueviewIarParentalStatus => {
                    "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS"
                }
                FilterPairType::FilterTrueviewIarRegion => "FILTER_TRUEVIEW_IAR_REGION",
                FilterPairType::FilterTrueviewIarRemarketingList => {
                    "FILTER_TRUEVIEW_IAR_REMARKETING_LIST"
                }
                FilterPairType::FilterTrueviewIarTimeOfDay => "FILTER_TRUEVIEW_IAR_TIME_OF_DAY",
                FilterPairType::FilterTrueviewIarYoutubeChannel => {
                    "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL"
                }
                FilterPairType::FilterTrueviewIarYoutubeVideo => {
                    "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO"
                }
                FilterPairType::FilterTrueviewIarZipcode => "FILTER_TRUEVIEW_IAR_ZIPCODE",
                FilterPairType::FilterTrueviewInterest => "FILTER_TRUEVIEW_INTEREST",
                FilterPairType::FilterTrueviewKeyword => "FILTER_TRUEVIEW_KEYWORD",
                FilterPairType::FilterTrueviewParentalStatus => "FILTER_TRUEVIEW_PARENTAL_STATUS",
                FilterPairType::FilterTrueviewPlacement => "FILTER_TRUEVIEW_PLACEMENT",
                FilterPairType::FilterTrueviewRegion => "FILTER_TRUEVIEW_REGION",
                FilterPairType::FilterTrueviewRemarketingList => "FILTER_TRUEVIEW_REMARKETING_LIST",
                FilterPairType::FilterTrueviewUrl => "FILTER_TRUEVIEW_URL",
                FilterPairType::FilterTrueviewZipcode => "FILTER_TRUEVIEW_ZIPCODE",
                FilterPairType::FilterUnknown => "FILTER_UNKNOWN",
                FilterPairType::FilterUserList => "FILTER_USER_LIST",
                FilterPairType::FilterUserListFirstParty => "FILTER_USER_LIST_FIRST_PARTY",
                FilterPairType::FilterUserListThirdParty => "FILTER_USER_LIST_THIRD_PARTY",
                FilterPairType::FilterVideoAdPositionInStream => {
                    "FILTER_VIDEO_AD_POSITION_IN_STREAM"
                }
                FilterPairType::FilterVideoCompanionSize => "FILTER_VIDEO_COMPANION_SIZE",
                FilterPairType::FilterVideoCompanionType => "FILTER_VIDEO_COMPANION_TYPE",
                FilterPairType::FilterVideoCreativeDuration => "FILTER_VIDEO_CREATIVE_DURATION",
                FilterPairType::FilterVideoCreativeDurationSkippable => {
                    "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE"
                }
                FilterPairType::FilterVideoDurationSeconds => "FILTER_VIDEO_DURATION_SECONDS",
                FilterPairType::FilterVideoDurationSecondsRange => {
                    "FILTER_VIDEO_DURATION_SECONDS_RANGE"
                }
                FilterPairType::FilterVideoFormatSupport => "FILTER_VIDEO_FORMAT_SUPPORT",
                FilterPairType::FilterVideoInventoryType => "FILTER_VIDEO_INVENTORY_TYPE",
                FilterPairType::FilterVideoPlayerSize => "FILTER_VIDEO_PLAYER_SIZE",
                FilterPairType::FilterVideoRatingTier => "FILTER_VIDEO_RATING_TIER",
                FilterPairType::FilterVideoSkippableSupport => "FILTER_VIDEO_SKIPPABLE_SUPPORT",
                FilterPairType::FilterVideoVpaidSupport => "FILTER_VIDEO_VPAID_SUPPORT",
                FilterPairType::FilterWeek => "FILTER_WEEK",
                FilterPairType::FilterYear => "FILTER_YEAR",
                FilterPairType::FilterYoutubeVertical => "FILTER_YOUTUBE_VERTICAL",
                FilterPairType::FilterZipCode => "FILTER_ZIP_CODE",
            }
        }
    }
    impl ::std::fmt::Display for FilterPairType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterPairType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterPairType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => {
                    FilterPairType::FilterActiveViewExpectedViewability
                }
                "FILTER_ACTIVITY_ID" => FilterPairType::FilterActivityId,
                "FILTER_ADVERTISER" => FilterPairType::FilterAdvertiser,
                "FILTER_ADVERTISER_CURRENCY" => FilterPairType::FilterAdvertiserCurrency,
                "FILTER_ADVERTISER_TIMEZONE" => FilterPairType::FilterAdvertiserTimezone,
                "FILTER_AD_POSITION" => FilterPairType::FilterAdPosition,
                "FILTER_AGE" => FilterPairType::FilterAge,
                "FILTER_AUTHORIZED_SELLER_STATE" => FilterPairType::FilterAuthorizedSellerState,
                "FILTER_BRANDSAFE_CHANNEL_ID" => FilterPairType::FilterBrandsafeChannelId,
                "FILTER_BROWSER" => FilterPairType::FilterBrowser,
                "FILTER_BUDGET_SEGMENT_DESCRIPTION" => {
                    FilterPairType::FilterBudgetSegmentDescription
                }
                "FILTER_CAMPAIGN_DAILY_FREQUENCY" => FilterPairType::FilterCampaignDailyFrequency,
                "FILTER_CARRIER" => FilterPairType::FilterCarrier,
                "FILTER_CHANNEL_ID" => FilterPairType::FilterChannelId,
                "FILTER_CITY" => FilterPairType::FilterCity,
                "FILTER_COMPANION_CREATIVE_ID" => FilterPairType::FilterCompanionCreativeId,
                "FILTER_CONVERSION_DELAY" => FilterPairType::FilterConversionDelay,
                "FILTER_COUNTRY" => FilterPairType::FilterCountry,
                "FILTER_CREATIVE_ATTRIBUTE" => FilterPairType::FilterCreativeAttribute,
                "FILTER_CREATIVE_HEIGHT" => FilterPairType::FilterCreativeHeight,
                "FILTER_CREATIVE_ID" => FilterPairType::FilterCreativeId,
                "FILTER_CREATIVE_SIZE" => FilterPairType::FilterCreativeSize,
                "FILTER_CREATIVE_TYPE" => FilterPairType::FilterCreativeType,
                "FILTER_CREATIVE_WIDTH" => FilterPairType::FilterCreativeWidth,
                "FILTER_DATA_PROVIDER" => FilterPairType::FilterDataProvider,
                "FILTER_DATE" => FilterPairType::FilterDate,
                "FILTER_DAY_OF_WEEK" => FilterPairType::FilterDayOfWeek,
                "FILTER_DEVICE_MAKE" => FilterPairType::FilterDeviceMake,
                "FILTER_DEVICE_MODEL" => FilterPairType::FilterDeviceModel,
                "FILTER_DEVICE_TYPE" => FilterPairType::FilterDeviceType,
                "FILTER_DFP_ORDER_ID" => FilterPairType::FilterDfpOrderId,
                "FILTER_DMA" => FilterPairType::FilterDma,
                "FILTER_EXCHANGE_ID" => FilterPairType::FilterExchangeId,
                "FILTER_FLOODLIGHT_ACTIVITY_ID" => FilterPairType::FilterFloodlightActivityId,
                "FILTER_GENDER" => FilterPairType::FilterGender,
                "FILTER_INSERTION_ORDER" => FilterPairType::FilterInsertionOrder,
                "FILTER_INVENTORY_COMMITMENT_TYPE" => FilterPairType::FilterInventoryCommitmentType,
                "FILTER_INVENTORY_DELIVERY_METHOD" => FilterPairType::FilterInventoryDeliveryMethod,
                "FILTER_INVENTORY_FORMAT" => FilterPairType::FilterInventoryFormat,
                "FILTER_INVENTORY_RATE_TYPE" => FilterPairType::FilterInventoryRateType,
                "FILTER_INVENTORY_SOURCE" => FilterPairType::FilterInventorySource,
                "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => {
                    FilterPairType::FilterInventorySourceExternalId
                }
                "FILTER_INVENTORY_SOURCE_TYPE" => FilterPairType::FilterInventorySourceType,
                "FILTER_KEYWORD" => FilterPairType::FilterKeyword,
                "FILTER_LINE_ITEM" => FilterPairType::FilterLineItem,
                "FILTER_LINE_ITEM_DAILY_FREQUENCY" => FilterPairType::FilterLineItemDailyFrequency,
                "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => {
                    FilterPairType::FilterLineItemLifetimeFrequency
                }
                "FILTER_LINE_ITEM_TYPE" => FilterPairType::FilterLineItemType,
                "FILTER_MEDIA_PLAN" => FilterPairType::FilterMediaPlan,
                "FILTER_MOBILE_DEVICE_MAKE" => FilterPairType::FilterMobileDeviceMake,
                "FILTER_MOBILE_DEVICE_MAKE_MODEL" => FilterPairType::FilterMobileDeviceMakeModel,
                "FILTER_MOBILE_DEVICE_TYPE" => FilterPairType::FilterMobileDeviceType,
                "FILTER_MOBILE_GEO" => FilterPairType::FilterMobileGeo,
                "FILTER_MONTH" => FilterPairType::FilterMonth,
                "FILTER_MRAID_SUPPORT" => FilterPairType::FilterMraidSupport,
                "FILTER_NIELSEN_AGE" => FilterPairType::FilterNielsenAge,
                "FILTER_NIELSEN_COUNTRY_CODE" => FilterPairType::FilterNielsenCountryCode,
                "FILTER_NIELSEN_DEVICE_ID" => FilterPairType::FilterNielsenDeviceId,
                "FILTER_NIELSEN_GENDER" => FilterPairType::FilterNielsenGender,
                "FILTER_NOT_SUPPORTED" => FilterPairType::FilterNotSupported,
                "FILTER_ORDER_ID" => FilterPairType::FilterOrderId,
                "FILTER_OS" => FilterPairType::FilterOs,
                "FILTER_PAGE_CATEGORY" => FilterPairType::FilterPageCategory,
                "FILTER_PAGE_LAYOUT" => FilterPairType::FilterPageLayout,
                "FILTER_PARTNER" => FilterPairType::FilterPartner,
                "FILTER_PARTNER_CURRENCY" => FilterPairType::FilterPartnerCurrency,
                "FILTER_PUBLIC_INVENTORY" => FilterPairType::FilterPublicInventory,
                "FILTER_QUARTER" => FilterPairType::FilterQuarter,
                "FILTER_REGION" => FilterPairType::FilterRegion,
                "FILTER_REGULAR_CHANNEL_ID" => FilterPairType::FilterRegularChannelId,
                "FILTER_SITE_ID" => FilterPairType::FilterSiteId,
                "FILTER_SITE_LANGUAGE" => FilterPairType::FilterSiteLanguage,
                "FILTER_SKIPPABLE_SUPPORT" => FilterPairType::FilterSkippableSupport,
                "FILTER_TARGETED_USER_LIST" => FilterPairType::FilterTargetedUserList,
                "FILTER_TIME_OF_DAY" => FilterPairType::FilterTimeOfDay,
                "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => FilterPairType::FilterTrueviewAdGroupAdId,
                "FILTER_TRUEVIEW_AD_GROUP_ID" => FilterPairType::FilterTrueviewAdGroupId,
                "FILTER_TRUEVIEW_AGE" => FilterPairType::FilterTrueviewAge,
                "FILTER_TRUEVIEW_CATEGORY" => FilterPairType::FilterTrueviewCategory,
                "FILTER_TRUEVIEW_CITY" => FilterPairType::FilterTrueviewCity,
                "FILTER_TRUEVIEW_CONVERSION_TYPE" => FilterPairType::FilterTrueviewConversionType,
                "FILTER_TRUEVIEW_COUNTRY" => FilterPairType::FilterTrueviewCountry,
                "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => FilterPairType::FilterTrueviewCustomAffinity,
                "FILTER_TRUEVIEW_DMA" => FilterPairType::FilterTrueviewDma,
                "FILTER_TRUEVIEW_GENDER" => FilterPairType::FilterTrueviewGender,
                "FILTER_TRUEVIEW_IAR_AGE" => FilterPairType::FilterTrueviewIarAge,
                "FILTER_TRUEVIEW_IAR_CATEGORY" => FilterPairType::FilterTrueviewIarCategory,
                "FILTER_TRUEVIEW_IAR_CITY" => FilterPairType::FilterTrueviewIarCity,
                "FILTER_TRUEVIEW_IAR_COUNTRY" => FilterPairType::FilterTrueviewIarCountry,
                "FILTER_TRUEVIEW_IAR_GENDER" => FilterPairType::FilterTrueviewIarGender,
                "FILTER_TRUEVIEW_IAR_INTEREST" => FilterPairType::FilterTrueviewIarInterest,
                "FILTER_TRUEVIEW_IAR_LANGUAGE" => FilterPairType::FilterTrueviewIarLanguage,
                "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => {
                    FilterPairType::FilterTrueviewIarParentalStatus
                }
                "FILTER_TRUEVIEW_IAR_REGION" => FilterPairType::FilterTrueviewIarRegion,
                "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => {
                    FilterPairType::FilterTrueviewIarRemarketingList
                }
                "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => FilterPairType::FilterTrueviewIarTimeOfDay,
                "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => {
                    FilterPairType::FilterTrueviewIarYoutubeChannel
                }
                "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => {
                    FilterPairType::FilterTrueviewIarYoutubeVideo
                }
                "FILTER_TRUEVIEW_IAR_ZIPCODE" => FilterPairType::FilterTrueviewIarZipcode,
                "FILTER_TRUEVIEW_INTEREST" => FilterPairType::FilterTrueviewInterest,
                "FILTER_TRUEVIEW_KEYWORD" => FilterPairType::FilterTrueviewKeyword,
                "FILTER_TRUEVIEW_PARENTAL_STATUS" => FilterPairType::FilterTrueviewParentalStatus,
                "FILTER_TRUEVIEW_PLACEMENT" => FilterPairType::FilterTrueviewPlacement,
                "FILTER_TRUEVIEW_REGION" => FilterPairType::FilterTrueviewRegion,
                "FILTER_TRUEVIEW_REMARKETING_LIST" => FilterPairType::FilterTrueviewRemarketingList,
                "FILTER_TRUEVIEW_URL" => FilterPairType::FilterTrueviewUrl,
                "FILTER_TRUEVIEW_ZIPCODE" => FilterPairType::FilterTrueviewZipcode,
                "FILTER_UNKNOWN" => FilterPairType::FilterUnknown,
                "FILTER_USER_LIST" => FilterPairType::FilterUserList,
                "FILTER_USER_LIST_FIRST_PARTY" => FilterPairType::FilterUserListFirstParty,
                "FILTER_USER_LIST_THIRD_PARTY" => FilterPairType::FilterUserListThirdParty,
                "FILTER_VIDEO_AD_POSITION_IN_STREAM" => {
                    FilterPairType::FilterVideoAdPositionInStream
                }
                "FILTER_VIDEO_COMPANION_SIZE" => FilterPairType::FilterVideoCompanionSize,
                "FILTER_VIDEO_COMPANION_TYPE" => FilterPairType::FilterVideoCompanionType,
                "FILTER_VIDEO_CREATIVE_DURATION" => FilterPairType::FilterVideoCreativeDuration,
                "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => {
                    FilterPairType::FilterVideoCreativeDurationSkippable
                }
                "FILTER_VIDEO_DURATION_SECONDS" => FilterPairType::FilterVideoDurationSeconds,
                "FILTER_VIDEO_DURATION_SECONDS_RANGE" => {
                    FilterPairType::FilterVideoDurationSecondsRange
                }
                "FILTER_VIDEO_FORMAT_SUPPORT" => FilterPairType::FilterVideoFormatSupport,
                "FILTER_VIDEO_INVENTORY_TYPE" => FilterPairType::FilterVideoInventoryType,
                "FILTER_VIDEO_PLAYER_SIZE" => FilterPairType::FilterVideoPlayerSize,
                "FILTER_VIDEO_RATING_TIER" => FilterPairType::FilterVideoRatingTier,
                "FILTER_VIDEO_SKIPPABLE_SUPPORT" => FilterPairType::FilterVideoSkippableSupport,
                "FILTER_VIDEO_VPAID_SUPPORT" => FilterPairType::FilterVideoVpaidSupport,
                "FILTER_WEEK" => FilterPairType::FilterWeek,
                "FILTER_YEAR" => FilterPairType::FilterYear,
                "FILTER_YOUTUBE_VERTICAL" => FilterPairType::FilterYoutubeVertical,
                "FILTER_ZIP_CODE" => FilterPairType::FilterZipCode,
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
    pub struct FilterPair {
        #[doc = "Filter type."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::FilterPairType>,
        #[doc = "Filter value."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for FilterPair {
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
    pub struct ListQueriesResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#listQueriesResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Retrieved queries."]
        #[serde(rename = "queries", default)]
        pub queries: Option<Vec<crate::schemas::Query>>,
    }
    impl ::field_selector::FieldSelector for ListQueriesResponse {
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
    pub struct ListReportsResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#listReportsResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Retrieved reports."]
        #[serde(rename = "reports", default)]
        pub reports: Option<Vec<crate::schemas::Report>>,
    }
    impl ::field_selector::FieldSelector for ListReportsResponse {
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
    pub enum ParametersGroupBysItems {
        FilterActiveViewExpectedViewability,
        FilterActivityId,
        FilterAdvertiser,
        FilterAdvertiserCurrency,
        FilterAdvertiserTimezone,
        FilterAdPosition,
        FilterAge,
        FilterAuthorizedSellerState,
        FilterBrandsafeChannelId,
        FilterBrowser,
        FilterBudgetSegmentDescription,
        FilterCampaignDailyFrequency,
        FilterCarrier,
        FilterChannelId,
        FilterCity,
        FilterCompanionCreativeId,
        FilterConversionDelay,
        FilterCountry,
        FilterCreativeAttribute,
        FilterCreativeHeight,
        FilterCreativeId,
        FilterCreativeSize,
        FilterCreativeType,
        FilterCreativeWidth,
        FilterDataProvider,
        FilterDate,
        FilterDayOfWeek,
        FilterDeviceMake,
        FilterDeviceModel,
        FilterDeviceType,
        FilterDfpOrderId,
        FilterDma,
        FilterExchangeId,
        FilterFloodlightActivityId,
        FilterGender,
        FilterInsertionOrder,
        FilterInventoryCommitmentType,
        FilterInventoryDeliveryMethod,
        FilterInventoryFormat,
        FilterInventoryRateType,
        FilterInventorySource,
        FilterInventorySourceExternalId,
        FilterInventorySourceType,
        FilterKeyword,
        FilterLineItem,
        FilterLineItemDailyFrequency,
        FilterLineItemLifetimeFrequency,
        FilterLineItemType,
        FilterMediaPlan,
        FilterMobileDeviceMake,
        FilterMobileDeviceMakeModel,
        FilterMobileDeviceType,
        FilterMobileGeo,
        FilterMonth,
        FilterMraidSupport,
        FilterNielsenAge,
        FilterNielsenCountryCode,
        FilterNielsenDeviceId,
        FilterNielsenGender,
        FilterNotSupported,
        FilterOrderId,
        FilterOs,
        FilterPageCategory,
        FilterPageLayout,
        FilterPartner,
        FilterPartnerCurrency,
        FilterPublicInventory,
        FilterQuarter,
        FilterRegion,
        FilterRegularChannelId,
        FilterSiteId,
        FilterSiteLanguage,
        FilterSkippableSupport,
        FilterTargetedUserList,
        FilterTimeOfDay,
        FilterTrueviewAdGroupAdId,
        FilterTrueviewAdGroupId,
        FilterTrueviewAge,
        FilterTrueviewCategory,
        FilterTrueviewCity,
        FilterTrueviewConversionType,
        FilterTrueviewCountry,
        FilterTrueviewCustomAffinity,
        FilterTrueviewDma,
        FilterTrueviewGender,
        FilterTrueviewIarAge,
        FilterTrueviewIarCategory,
        FilterTrueviewIarCity,
        FilterTrueviewIarCountry,
        FilterTrueviewIarGender,
        FilterTrueviewIarInterest,
        FilterTrueviewIarLanguage,
        FilterTrueviewIarParentalStatus,
        FilterTrueviewIarRegion,
        FilterTrueviewIarRemarketingList,
        FilterTrueviewIarTimeOfDay,
        FilterTrueviewIarYoutubeChannel,
        FilterTrueviewIarYoutubeVideo,
        FilterTrueviewIarZipcode,
        FilterTrueviewInterest,
        FilterTrueviewKeyword,
        FilterTrueviewParentalStatus,
        FilterTrueviewPlacement,
        FilterTrueviewRegion,
        FilterTrueviewRemarketingList,
        FilterTrueviewUrl,
        FilterTrueviewZipcode,
        FilterUnknown,
        FilterUserList,
        FilterUserListFirstParty,
        FilterUserListThirdParty,
        FilterVideoAdPositionInStream,
        FilterVideoCompanionSize,
        FilterVideoCompanionType,
        FilterVideoCreativeDuration,
        FilterVideoCreativeDurationSkippable,
        FilterVideoDurationSeconds,
        FilterVideoDurationSecondsRange,
        FilterVideoFormatSupport,
        FilterVideoInventoryType,
        FilterVideoPlayerSize,
        FilterVideoRatingTier,
        FilterVideoSkippableSupport,
        FilterVideoVpaidSupport,
        FilterWeek,
        FilterYear,
        FilterYoutubeVertical,
        FilterZipCode,
    }
    impl ParametersGroupBysItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ParametersGroupBysItems::FilterActiveViewExpectedViewability => {
                    "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY"
                }
                ParametersGroupBysItems::FilterActivityId => "FILTER_ACTIVITY_ID",
                ParametersGroupBysItems::FilterAdvertiser => "FILTER_ADVERTISER",
                ParametersGroupBysItems::FilterAdvertiserCurrency => "FILTER_ADVERTISER_CURRENCY",
                ParametersGroupBysItems::FilterAdvertiserTimezone => "FILTER_ADVERTISER_TIMEZONE",
                ParametersGroupBysItems::FilterAdPosition => "FILTER_AD_POSITION",
                ParametersGroupBysItems::FilterAge => "FILTER_AGE",
                ParametersGroupBysItems::FilterAuthorizedSellerState => {
                    "FILTER_AUTHORIZED_SELLER_STATE"
                }
                ParametersGroupBysItems::FilterBrandsafeChannelId => "FILTER_BRANDSAFE_CHANNEL_ID",
                ParametersGroupBysItems::FilterBrowser => "FILTER_BROWSER",
                ParametersGroupBysItems::FilterBudgetSegmentDescription => {
                    "FILTER_BUDGET_SEGMENT_DESCRIPTION"
                }
                ParametersGroupBysItems::FilterCampaignDailyFrequency => {
                    "FILTER_CAMPAIGN_DAILY_FREQUENCY"
                }
                ParametersGroupBysItems::FilterCarrier => "FILTER_CARRIER",
                ParametersGroupBysItems::FilterChannelId => "FILTER_CHANNEL_ID",
                ParametersGroupBysItems::FilterCity => "FILTER_CITY",
                ParametersGroupBysItems::FilterCompanionCreativeId => {
                    "FILTER_COMPANION_CREATIVE_ID"
                }
                ParametersGroupBysItems::FilterConversionDelay => "FILTER_CONVERSION_DELAY",
                ParametersGroupBysItems::FilterCountry => "FILTER_COUNTRY",
                ParametersGroupBysItems::FilterCreativeAttribute => "FILTER_CREATIVE_ATTRIBUTE",
                ParametersGroupBysItems::FilterCreativeHeight => "FILTER_CREATIVE_HEIGHT",
                ParametersGroupBysItems::FilterCreativeId => "FILTER_CREATIVE_ID",
                ParametersGroupBysItems::FilterCreativeSize => "FILTER_CREATIVE_SIZE",
                ParametersGroupBysItems::FilterCreativeType => "FILTER_CREATIVE_TYPE",
                ParametersGroupBysItems::FilterCreativeWidth => "FILTER_CREATIVE_WIDTH",
                ParametersGroupBysItems::FilterDataProvider => "FILTER_DATA_PROVIDER",
                ParametersGroupBysItems::FilterDate => "FILTER_DATE",
                ParametersGroupBysItems::FilterDayOfWeek => "FILTER_DAY_OF_WEEK",
                ParametersGroupBysItems::FilterDeviceMake => "FILTER_DEVICE_MAKE",
                ParametersGroupBysItems::FilterDeviceModel => "FILTER_DEVICE_MODEL",
                ParametersGroupBysItems::FilterDeviceType => "FILTER_DEVICE_TYPE",
                ParametersGroupBysItems::FilterDfpOrderId => "FILTER_DFP_ORDER_ID",
                ParametersGroupBysItems::FilterDma => "FILTER_DMA",
                ParametersGroupBysItems::FilterExchangeId => "FILTER_EXCHANGE_ID",
                ParametersGroupBysItems::FilterFloodlightActivityId => {
                    "FILTER_FLOODLIGHT_ACTIVITY_ID"
                }
                ParametersGroupBysItems::FilterGender => "FILTER_GENDER",
                ParametersGroupBysItems::FilterInsertionOrder => "FILTER_INSERTION_ORDER",
                ParametersGroupBysItems::FilterInventoryCommitmentType => {
                    "FILTER_INVENTORY_COMMITMENT_TYPE"
                }
                ParametersGroupBysItems::FilterInventoryDeliveryMethod => {
                    "FILTER_INVENTORY_DELIVERY_METHOD"
                }
                ParametersGroupBysItems::FilterInventoryFormat => "FILTER_INVENTORY_FORMAT",
                ParametersGroupBysItems::FilterInventoryRateType => "FILTER_INVENTORY_RATE_TYPE",
                ParametersGroupBysItems::FilterInventorySource => "FILTER_INVENTORY_SOURCE",
                ParametersGroupBysItems::FilterInventorySourceExternalId => {
                    "FILTER_INVENTORY_SOURCE_EXTERNAL_ID"
                }
                ParametersGroupBysItems::FilterInventorySourceType => {
                    "FILTER_INVENTORY_SOURCE_TYPE"
                }
                ParametersGroupBysItems::FilterKeyword => "FILTER_KEYWORD",
                ParametersGroupBysItems::FilterLineItem => "FILTER_LINE_ITEM",
                ParametersGroupBysItems::FilterLineItemDailyFrequency => {
                    "FILTER_LINE_ITEM_DAILY_FREQUENCY"
                }
                ParametersGroupBysItems::FilterLineItemLifetimeFrequency => {
                    "FILTER_LINE_ITEM_LIFETIME_FREQUENCY"
                }
                ParametersGroupBysItems::FilterLineItemType => "FILTER_LINE_ITEM_TYPE",
                ParametersGroupBysItems::FilterMediaPlan => "FILTER_MEDIA_PLAN",
                ParametersGroupBysItems::FilterMobileDeviceMake => "FILTER_MOBILE_DEVICE_MAKE",
                ParametersGroupBysItems::FilterMobileDeviceMakeModel => {
                    "FILTER_MOBILE_DEVICE_MAKE_MODEL"
                }
                ParametersGroupBysItems::FilterMobileDeviceType => "FILTER_MOBILE_DEVICE_TYPE",
                ParametersGroupBysItems::FilterMobileGeo => "FILTER_MOBILE_GEO",
                ParametersGroupBysItems::FilterMonth => "FILTER_MONTH",
                ParametersGroupBysItems::FilterMraidSupport => "FILTER_MRAID_SUPPORT",
                ParametersGroupBysItems::FilterNielsenAge => "FILTER_NIELSEN_AGE",
                ParametersGroupBysItems::FilterNielsenCountryCode => "FILTER_NIELSEN_COUNTRY_CODE",
                ParametersGroupBysItems::FilterNielsenDeviceId => "FILTER_NIELSEN_DEVICE_ID",
                ParametersGroupBysItems::FilterNielsenGender => "FILTER_NIELSEN_GENDER",
                ParametersGroupBysItems::FilterNotSupported => "FILTER_NOT_SUPPORTED",
                ParametersGroupBysItems::FilterOrderId => "FILTER_ORDER_ID",
                ParametersGroupBysItems::FilterOs => "FILTER_OS",
                ParametersGroupBysItems::FilterPageCategory => "FILTER_PAGE_CATEGORY",
                ParametersGroupBysItems::FilterPageLayout => "FILTER_PAGE_LAYOUT",
                ParametersGroupBysItems::FilterPartner => "FILTER_PARTNER",
                ParametersGroupBysItems::FilterPartnerCurrency => "FILTER_PARTNER_CURRENCY",
                ParametersGroupBysItems::FilterPublicInventory => "FILTER_PUBLIC_INVENTORY",
                ParametersGroupBysItems::FilterQuarter => "FILTER_QUARTER",
                ParametersGroupBysItems::FilterRegion => "FILTER_REGION",
                ParametersGroupBysItems::FilterRegularChannelId => "FILTER_REGULAR_CHANNEL_ID",
                ParametersGroupBysItems::FilterSiteId => "FILTER_SITE_ID",
                ParametersGroupBysItems::FilterSiteLanguage => "FILTER_SITE_LANGUAGE",
                ParametersGroupBysItems::FilterSkippableSupport => "FILTER_SKIPPABLE_SUPPORT",
                ParametersGroupBysItems::FilterTargetedUserList => "FILTER_TARGETED_USER_LIST",
                ParametersGroupBysItems::FilterTimeOfDay => "FILTER_TIME_OF_DAY",
                ParametersGroupBysItems::FilterTrueviewAdGroupAdId => {
                    "FILTER_TRUEVIEW_AD_GROUP_AD_ID"
                }
                ParametersGroupBysItems::FilterTrueviewAdGroupId => "FILTER_TRUEVIEW_AD_GROUP_ID",
                ParametersGroupBysItems::FilterTrueviewAge => "FILTER_TRUEVIEW_AGE",
                ParametersGroupBysItems::FilterTrueviewCategory => "FILTER_TRUEVIEW_CATEGORY",
                ParametersGroupBysItems::FilterTrueviewCity => "FILTER_TRUEVIEW_CITY",
                ParametersGroupBysItems::FilterTrueviewConversionType => {
                    "FILTER_TRUEVIEW_CONVERSION_TYPE"
                }
                ParametersGroupBysItems::FilterTrueviewCountry => "FILTER_TRUEVIEW_COUNTRY",
                ParametersGroupBysItems::FilterTrueviewCustomAffinity => {
                    "FILTER_TRUEVIEW_CUSTOM_AFFINITY"
                }
                ParametersGroupBysItems::FilterTrueviewDma => "FILTER_TRUEVIEW_DMA",
                ParametersGroupBysItems::FilterTrueviewGender => "FILTER_TRUEVIEW_GENDER",
                ParametersGroupBysItems::FilterTrueviewIarAge => "FILTER_TRUEVIEW_IAR_AGE",
                ParametersGroupBysItems::FilterTrueviewIarCategory => {
                    "FILTER_TRUEVIEW_IAR_CATEGORY"
                }
                ParametersGroupBysItems::FilterTrueviewIarCity => "FILTER_TRUEVIEW_IAR_CITY",
                ParametersGroupBysItems::FilterTrueviewIarCountry => "FILTER_TRUEVIEW_IAR_COUNTRY",
                ParametersGroupBysItems::FilterTrueviewIarGender => "FILTER_TRUEVIEW_IAR_GENDER",
                ParametersGroupBysItems::FilterTrueviewIarInterest => {
                    "FILTER_TRUEVIEW_IAR_INTEREST"
                }
                ParametersGroupBysItems::FilterTrueviewIarLanguage => {
                    "FILTER_TRUEVIEW_IAR_LANGUAGE"
                }
                ParametersGroupBysItems::FilterTrueviewIarParentalStatus => {
                    "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS"
                }
                ParametersGroupBysItems::FilterTrueviewIarRegion => "FILTER_TRUEVIEW_IAR_REGION",
                ParametersGroupBysItems::FilterTrueviewIarRemarketingList => {
                    "FILTER_TRUEVIEW_IAR_REMARKETING_LIST"
                }
                ParametersGroupBysItems::FilterTrueviewIarTimeOfDay => {
                    "FILTER_TRUEVIEW_IAR_TIME_OF_DAY"
                }
                ParametersGroupBysItems::FilterTrueviewIarYoutubeChannel => {
                    "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL"
                }
                ParametersGroupBysItems::FilterTrueviewIarYoutubeVideo => {
                    "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO"
                }
                ParametersGroupBysItems::FilterTrueviewIarZipcode => "FILTER_TRUEVIEW_IAR_ZIPCODE",
                ParametersGroupBysItems::FilterTrueviewInterest => "FILTER_TRUEVIEW_INTEREST",
                ParametersGroupBysItems::FilterTrueviewKeyword => "FILTER_TRUEVIEW_KEYWORD",
                ParametersGroupBysItems::FilterTrueviewParentalStatus => {
                    "FILTER_TRUEVIEW_PARENTAL_STATUS"
                }
                ParametersGroupBysItems::FilterTrueviewPlacement => "FILTER_TRUEVIEW_PLACEMENT",
                ParametersGroupBysItems::FilterTrueviewRegion => "FILTER_TRUEVIEW_REGION",
                ParametersGroupBysItems::FilterTrueviewRemarketingList => {
                    "FILTER_TRUEVIEW_REMARKETING_LIST"
                }
                ParametersGroupBysItems::FilterTrueviewUrl => "FILTER_TRUEVIEW_URL",
                ParametersGroupBysItems::FilterTrueviewZipcode => "FILTER_TRUEVIEW_ZIPCODE",
                ParametersGroupBysItems::FilterUnknown => "FILTER_UNKNOWN",
                ParametersGroupBysItems::FilterUserList => "FILTER_USER_LIST",
                ParametersGroupBysItems::FilterUserListFirstParty => "FILTER_USER_LIST_FIRST_PARTY",
                ParametersGroupBysItems::FilterUserListThirdParty => "FILTER_USER_LIST_THIRD_PARTY",
                ParametersGroupBysItems::FilterVideoAdPositionInStream => {
                    "FILTER_VIDEO_AD_POSITION_IN_STREAM"
                }
                ParametersGroupBysItems::FilterVideoCompanionSize => "FILTER_VIDEO_COMPANION_SIZE",
                ParametersGroupBysItems::FilterVideoCompanionType => "FILTER_VIDEO_COMPANION_TYPE",
                ParametersGroupBysItems::FilterVideoCreativeDuration => {
                    "FILTER_VIDEO_CREATIVE_DURATION"
                }
                ParametersGroupBysItems::FilterVideoCreativeDurationSkippable => {
                    "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE"
                }
                ParametersGroupBysItems::FilterVideoDurationSeconds => {
                    "FILTER_VIDEO_DURATION_SECONDS"
                }
                ParametersGroupBysItems::FilterVideoDurationSecondsRange => {
                    "FILTER_VIDEO_DURATION_SECONDS_RANGE"
                }
                ParametersGroupBysItems::FilterVideoFormatSupport => "FILTER_VIDEO_FORMAT_SUPPORT",
                ParametersGroupBysItems::FilterVideoInventoryType => "FILTER_VIDEO_INVENTORY_TYPE",
                ParametersGroupBysItems::FilterVideoPlayerSize => "FILTER_VIDEO_PLAYER_SIZE",
                ParametersGroupBysItems::FilterVideoRatingTier => "FILTER_VIDEO_RATING_TIER",
                ParametersGroupBysItems::FilterVideoSkippableSupport => {
                    "FILTER_VIDEO_SKIPPABLE_SUPPORT"
                }
                ParametersGroupBysItems::FilterVideoVpaidSupport => "FILTER_VIDEO_VPAID_SUPPORT",
                ParametersGroupBysItems::FilterWeek => "FILTER_WEEK",
                ParametersGroupBysItems::FilterYear => "FILTER_YEAR",
                ParametersGroupBysItems::FilterYoutubeVertical => "FILTER_YOUTUBE_VERTICAL",
                ParametersGroupBysItems::FilterZipCode => "FILTER_ZIP_CODE",
            }
        }
    }
    impl ::std::fmt::Display for ParametersGroupBysItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParametersGroupBysItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParametersGroupBysItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => {
                    ParametersGroupBysItems::FilterActiveViewExpectedViewability
                }
                "FILTER_ACTIVITY_ID" => ParametersGroupBysItems::FilterActivityId,
                "FILTER_ADVERTISER" => ParametersGroupBysItems::FilterAdvertiser,
                "FILTER_ADVERTISER_CURRENCY" => ParametersGroupBysItems::FilterAdvertiserCurrency,
                "FILTER_ADVERTISER_TIMEZONE" => ParametersGroupBysItems::FilterAdvertiserTimezone,
                "FILTER_AD_POSITION" => ParametersGroupBysItems::FilterAdPosition,
                "FILTER_AGE" => ParametersGroupBysItems::FilterAge,
                "FILTER_AUTHORIZED_SELLER_STATE" => {
                    ParametersGroupBysItems::FilterAuthorizedSellerState
                }
                "FILTER_BRANDSAFE_CHANNEL_ID" => ParametersGroupBysItems::FilterBrandsafeChannelId,
                "FILTER_BROWSER" => ParametersGroupBysItems::FilterBrowser,
                "FILTER_BUDGET_SEGMENT_DESCRIPTION" => {
                    ParametersGroupBysItems::FilterBudgetSegmentDescription
                }
                "FILTER_CAMPAIGN_DAILY_FREQUENCY" => {
                    ParametersGroupBysItems::FilterCampaignDailyFrequency
                }
                "FILTER_CARRIER" => ParametersGroupBysItems::FilterCarrier,
                "FILTER_CHANNEL_ID" => ParametersGroupBysItems::FilterChannelId,
                "FILTER_CITY" => ParametersGroupBysItems::FilterCity,
                "FILTER_COMPANION_CREATIVE_ID" => {
                    ParametersGroupBysItems::FilterCompanionCreativeId
                }
                "FILTER_CONVERSION_DELAY" => ParametersGroupBysItems::FilterConversionDelay,
                "FILTER_COUNTRY" => ParametersGroupBysItems::FilterCountry,
                "FILTER_CREATIVE_ATTRIBUTE" => ParametersGroupBysItems::FilterCreativeAttribute,
                "FILTER_CREATIVE_HEIGHT" => ParametersGroupBysItems::FilterCreativeHeight,
                "FILTER_CREATIVE_ID" => ParametersGroupBysItems::FilterCreativeId,
                "FILTER_CREATIVE_SIZE" => ParametersGroupBysItems::FilterCreativeSize,
                "FILTER_CREATIVE_TYPE" => ParametersGroupBysItems::FilterCreativeType,
                "FILTER_CREATIVE_WIDTH" => ParametersGroupBysItems::FilterCreativeWidth,
                "FILTER_DATA_PROVIDER" => ParametersGroupBysItems::FilterDataProvider,
                "FILTER_DATE" => ParametersGroupBysItems::FilterDate,
                "FILTER_DAY_OF_WEEK" => ParametersGroupBysItems::FilterDayOfWeek,
                "FILTER_DEVICE_MAKE" => ParametersGroupBysItems::FilterDeviceMake,
                "FILTER_DEVICE_MODEL" => ParametersGroupBysItems::FilterDeviceModel,
                "FILTER_DEVICE_TYPE" => ParametersGroupBysItems::FilterDeviceType,
                "FILTER_DFP_ORDER_ID" => ParametersGroupBysItems::FilterDfpOrderId,
                "FILTER_DMA" => ParametersGroupBysItems::FilterDma,
                "FILTER_EXCHANGE_ID" => ParametersGroupBysItems::FilterExchangeId,
                "FILTER_FLOODLIGHT_ACTIVITY_ID" => {
                    ParametersGroupBysItems::FilterFloodlightActivityId
                }
                "FILTER_GENDER" => ParametersGroupBysItems::FilterGender,
                "FILTER_INSERTION_ORDER" => ParametersGroupBysItems::FilterInsertionOrder,
                "FILTER_INVENTORY_COMMITMENT_TYPE" => {
                    ParametersGroupBysItems::FilterInventoryCommitmentType
                }
                "FILTER_INVENTORY_DELIVERY_METHOD" => {
                    ParametersGroupBysItems::FilterInventoryDeliveryMethod
                }
                "FILTER_INVENTORY_FORMAT" => ParametersGroupBysItems::FilterInventoryFormat,
                "FILTER_INVENTORY_RATE_TYPE" => ParametersGroupBysItems::FilterInventoryRateType,
                "FILTER_INVENTORY_SOURCE" => ParametersGroupBysItems::FilterInventorySource,
                "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => {
                    ParametersGroupBysItems::FilterInventorySourceExternalId
                }
                "FILTER_INVENTORY_SOURCE_TYPE" => {
                    ParametersGroupBysItems::FilterInventorySourceType
                }
                "FILTER_KEYWORD" => ParametersGroupBysItems::FilterKeyword,
                "FILTER_LINE_ITEM" => ParametersGroupBysItems::FilterLineItem,
                "FILTER_LINE_ITEM_DAILY_FREQUENCY" => {
                    ParametersGroupBysItems::FilterLineItemDailyFrequency
                }
                "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => {
                    ParametersGroupBysItems::FilterLineItemLifetimeFrequency
                }
                "FILTER_LINE_ITEM_TYPE" => ParametersGroupBysItems::FilterLineItemType,
                "FILTER_MEDIA_PLAN" => ParametersGroupBysItems::FilterMediaPlan,
                "FILTER_MOBILE_DEVICE_MAKE" => ParametersGroupBysItems::FilterMobileDeviceMake,
                "FILTER_MOBILE_DEVICE_MAKE_MODEL" => {
                    ParametersGroupBysItems::FilterMobileDeviceMakeModel
                }
                "FILTER_MOBILE_DEVICE_TYPE" => ParametersGroupBysItems::FilterMobileDeviceType,
                "FILTER_MOBILE_GEO" => ParametersGroupBysItems::FilterMobileGeo,
                "FILTER_MONTH" => ParametersGroupBysItems::FilterMonth,
                "FILTER_MRAID_SUPPORT" => ParametersGroupBysItems::FilterMraidSupport,
                "FILTER_NIELSEN_AGE" => ParametersGroupBysItems::FilterNielsenAge,
                "FILTER_NIELSEN_COUNTRY_CODE" => ParametersGroupBysItems::FilterNielsenCountryCode,
                "FILTER_NIELSEN_DEVICE_ID" => ParametersGroupBysItems::FilterNielsenDeviceId,
                "FILTER_NIELSEN_GENDER" => ParametersGroupBysItems::FilterNielsenGender,
                "FILTER_NOT_SUPPORTED" => ParametersGroupBysItems::FilterNotSupported,
                "FILTER_ORDER_ID" => ParametersGroupBysItems::FilterOrderId,
                "FILTER_OS" => ParametersGroupBysItems::FilterOs,
                "FILTER_PAGE_CATEGORY" => ParametersGroupBysItems::FilterPageCategory,
                "FILTER_PAGE_LAYOUT" => ParametersGroupBysItems::FilterPageLayout,
                "FILTER_PARTNER" => ParametersGroupBysItems::FilterPartner,
                "FILTER_PARTNER_CURRENCY" => ParametersGroupBysItems::FilterPartnerCurrency,
                "FILTER_PUBLIC_INVENTORY" => ParametersGroupBysItems::FilterPublicInventory,
                "FILTER_QUARTER" => ParametersGroupBysItems::FilterQuarter,
                "FILTER_REGION" => ParametersGroupBysItems::FilterRegion,
                "FILTER_REGULAR_CHANNEL_ID" => ParametersGroupBysItems::FilterRegularChannelId,
                "FILTER_SITE_ID" => ParametersGroupBysItems::FilterSiteId,
                "FILTER_SITE_LANGUAGE" => ParametersGroupBysItems::FilterSiteLanguage,
                "FILTER_SKIPPABLE_SUPPORT" => ParametersGroupBysItems::FilterSkippableSupport,
                "FILTER_TARGETED_USER_LIST" => ParametersGroupBysItems::FilterTargetedUserList,
                "FILTER_TIME_OF_DAY" => ParametersGroupBysItems::FilterTimeOfDay,
                "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => {
                    ParametersGroupBysItems::FilterTrueviewAdGroupAdId
                }
                "FILTER_TRUEVIEW_AD_GROUP_ID" => ParametersGroupBysItems::FilterTrueviewAdGroupId,
                "FILTER_TRUEVIEW_AGE" => ParametersGroupBysItems::FilterTrueviewAge,
                "FILTER_TRUEVIEW_CATEGORY" => ParametersGroupBysItems::FilterTrueviewCategory,
                "FILTER_TRUEVIEW_CITY" => ParametersGroupBysItems::FilterTrueviewCity,
                "FILTER_TRUEVIEW_CONVERSION_TYPE" => {
                    ParametersGroupBysItems::FilterTrueviewConversionType
                }
                "FILTER_TRUEVIEW_COUNTRY" => ParametersGroupBysItems::FilterTrueviewCountry,
                "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => {
                    ParametersGroupBysItems::FilterTrueviewCustomAffinity
                }
                "FILTER_TRUEVIEW_DMA" => ParametersGroupBysItems::FilterTrueviewDma,
                "FILTER_TRUEVIEW_GENDER" => ParametersGroupBysItems::FilterTrueviewGender,
                "FILTER_TRUEVIEW_IAR_AGE" => ParametersGroupBysItems::FilterTrueviewIarAge,
                "FILTER_TRUEVIEW_IAR_CATEGORY" => {
                    ParametersGroupBysItems::FilterTrueviewIarCategory
                }
                "FILTER_TRUEVIEW_IAR_CITY" => ParametersGroupBysItems::FilterTrueviewIarCity,
                "FILTER_TRUEVIEW_IAR_COUNTRY" => ParametersGroupBysItems::FilterTrueviewIarCountry,
                "FILTER_TRUEVIEW_IAR_GENDER" => ParametersGroupBysItems::FilterTrueviewIarGender,
                "FILTER_TRUEVIEW_IAR_INTEREST" => {
                    ParametersGroupBysItems::FilterTrueviewIarInterest
                }
                "FILTER_TRUEVIEW_IAR_LANGUAGE" => {
                    ParametersGroupBysItems::FilterTrueviewIarLanguage
                }
                "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => {
                    ParametersGroupBysItems::FilterTrueviewIarParentalStatus
                }
                "FILTER_TRUEVIEW_IAR_REGION" => ParametersGroupBysItems::FilterTrueviewIarRegion,
                "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => {
                    ParametersGroupBysItems::FilterTrueviewIarRemarketingList
                }
                "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => {
                    ParametersGroupBysItems::FilterTrueviewIarTimeOfDay
                }
                "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => {
                    ParametersGroupBysItems::FilterTrueviewIarYoutubeChannel
                }
                "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => {
                    ParametersGroupBysItems::FilterTrueviewIarYoutubeVideo
                }
                "FILTER_TRUEVIEW_IAR_ZIPCODE" => ParametersGroupBysItems::FilterTrueviewIarZipcode,
                "FILTER_TRUEVIEW_INTEREST" => ParametersGroupBysItems::FilterTrueviewInterest,
                "FILTER_TRUEVIEW_KEYWORD" => ParametersGroupBysItems::FilterTrueviewKeyword,
                "FILTER_TRUEVIEW_PARENTAL_STATUS" => {
                    ParametersGroupBysItems::FilterTrueviewParentalStatus
                }
                "FILTER_TRUEVIEW_PLACEMENT" => ParametersGroupBysItems::FilterTrueviewPlacement,
                "FILTER_TRUEVIEW_REGION" => ParametersGroupBysItems::FilterTrueviewRegion,
                "FILTER_TRUEVIEW_REMARKETING_LIST" => {
                    ParametersGroupBysItems::FilterTrueviewRemarketingList
                }
                "FILTER_TRUEVIEW_URL" => ParametersGroupBysItems::FilterTrueviewUrl,
                "FILTER_TRUEVIEW_ZIPCODE" => ParametersGroupBysItems::FilterTrueviewZipcode,
                "FILTER_UNKNOWN" => ParametersGroupBysItems::FilterUnknown,
                "FILTER_USER_LIST" => ParametersGroupBysItems::FilterUserList,
                "FILTER_USER_LIST_FIRST_PARTY" => ParametersGroupBysItems::FilterUserListFirstParty,
                "FILTER_USER_LIST_THIRD_PARTY" => ParametersGroupBysItems::FilterUserListThirdParty,
                "FILTER_VIDEO_AD_POSITION_IN_STREAM" => {
                    ParametersGroupBysItems::FilterVideoAdPositionInStream
                }
                "FILTER_VIDEO_COMPANION_SIZE" => ParametersGroupBysItems::FilterVideoCompanionSize,
                "FILTER_VIDEO_COMPANION_TYPE" => ParametersGroupBysItems::FilterVideoCompanionType,
                "FILTER_VIDEO_CREATIVE_DURATION" => {
                    ParametersGroupBysItems::FilterVideoCreativeDuration
                }
                "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => {
                    ParametersGroupBysItems::FilterVideoCreativeDurationSkippable
                }
                "FILTER_VIDEO_DURATION_SECONDS" => {
                    ParametersGroupBysItems::FilterVideoDurationSeconds
                }
                "FILTER_VIDEO_DURATION_SECONDS_RANGE" => {
                    ParametersGroupBysItems::FilterVideoDurationSecondsRange
                }
                "FILTER_VIDEO_FORMAT_SUPPORT" => ParametersGroupBysItems::FilterVideoFormatSupport,
                "FILTER_VIDEO_INVENTORY_TYPE" => ParametersGroupBysItems::FilterVideoInventoryType,
                "FILTER_VIDEO_PLAYER_SIZE" => ParametersGroupBysItems::FilterVideoPlayerSize,
                "FILTER_VIDEO_RATING_TIER" => ParametersGroupBysItems::FilterVideoRatingTier,
                "FILTER_VIDEO_SKIPPABLE_SUPPORT" => {
                    ParametersGroupBysItems::FilterVideoSkippableSupport
                }
                "FILTER_VIDEO_VPAID_SUPPORT" => ParametersGroupBysItems::FilterVideoVpaidSupport,
                "FILTER_WEEK" => ParametersGroupBysItems::FilterWeek,
                "FILTER_YEAR" => ParametersGroupBysItems::FilterYear,
                "FILTER_YOUTUBE_VERTICAL" => ParametersGroupBysItems::FilterYoutubeVertical,
                "FILTER_ZIP_CODE" => ParametersGroupBysItems::FilterZipCode,
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
    pub enum ParametersMetricsItems {
        MetricActiveViewAudibleVisibleOnCompleteImpressions,
        MetricActiveViewAverageViewableTime,
        MetricActiveViewDistributionUnmeasurable,
        MetricActiveViewDistributionUnviewable,
        MetricActiveViewDistributionViewable,
        MetricActiveViewEligibleImpressions,
        MetricActiveViewMeasurableImpressions,
        MetricActiveViewPctMeasurableImpressions,
        MetricActiveViewPctViewableImpressions,
        MetricActiveViewPercentAudibleVisibleAtStart,
        MetricActiveViewPercentAudibleVisibleFirstQuar,
        MetricActiveViewPercentAudibleVisibleOnComplete,
        MetricActiveViewPercentAudibleVisibleSecondQuar,
        MetricActiveViewPercentAudibleVisibleThirdQuar,
        MetricActiveViewPercentViewableForTimeThreshold,
        MetricActiveViewPercentVisibleAtStart,
        MetricActiveViewPercentVisibleFirstQuar,
        MetricActiveViewPercentVisibleOnComplete,
        MetricActiveViewPercentVisibleSecondQuar,
        MetricActiveViewPercentVisibleThirdQuar,
        MetricActiveViewUnmeasurableImpressions,
        MetricActiveViewUnviewableImpressions,
        MetricActiveViewViewableForTimeThreshold,
        MetricActiveViewViewableImpressions,
        MetricBidRequests,
        MetricBillableCostAdvertiser,
        MetricBillableCostPartner,
        MetricBillableCostUsd,
        MetricBillableImpressions,
        MetricClicks,
        MetricClickToPostClickConversionRate,
        MetricCmPostClickRevenue,
        MetricCmPostViewRevenue,
        MetricComscoreVceAudienceAvgFrequency,
        MetricComscoreVceAudienceImpressions,
        MetricComscoreVceAudienceImpressionsShare,
        MetricComscoreVceAudienceReachPct,
        MetricComscoreVceAudienceSharePct,
        MetricComscoreVceGrossRatingPoints,
        MetricComscoreVcePopulation,
        MetricComscoreVceUniqueAudience,
        MetricConversionsPerMille,
        MetricCookieReachAverageImpressionFrequency,
        MetricCookieReachImpressionReach,
        MetricCpmFee1Advertiser,
        MetricCpmFee1Partner,
        MetricCpmFee1Usd,
        MetricCpmFee2Advertiser,
        MetricCpmFee2Partner,
        MetricCpmFee2Usd,
        MetricCpmFee3Advertiser,
        MetricCpmFee3Partner,
        MetricCpmFee3Usd,
        MetricCpmFee4Advertiser,
        MetricCpmFee4Partner,
        MetricCpmFee4Usd,
        MetricCpmFee5Advertiser,
        MetricCpmFee5Partner,
        MetricCpmFee5Usd,
        MetricCtr,
        MetricDataCostAdvertiser,
        MetricDataCostPartner,
        MetricDataCostUsd,
        MetricDbmEngagementRate,
        MetricFee10Advertiser,
        MetricFee10Partner,
        MetricFee10Usd,
        MetricFee11Advertiser,
        MetricFee11Partner,
        MetricFee11Usd,
        MetricFee12Advertiser,
        MetricFee12Partner,
        MetricFee12Usd,
        MetricFee13Advertiser,
        MetricFee13Partner,
        MetricFee13Usd,
        MetricFee14Advertiser,
        MetricFee14Partner,
        MetricFee14Usd,
        MetricFee15Advertiser,
        MetricFee15Partner,
        MetricFee15Usd,
        MetricFee16Advertiser,
        MetricFee16Partner,
        MetricFee16Usd,
        MetricFee17Advertiser,
        MetricFee17Partner,
        MetricFee17Usd,
        MetricFee18Advertiser,
        MetricFee18Partner,
        MetricFee18Usd,
        MetricFee19Advertiser,
        MetricFee19Partner,
        MetricFee19Usd,
        MetricFee20Advertiser,
        MetricFee20Partner,
        MetricFee20Usd,
        MetricFee21Advertiser,
        MetricFee21Partner,
        MetricFee21Usd,
        MetricFee22Advertiser,
        MetricFee22Partner,
        MetricFee22Usd,
        MetricFee2Advertiser,
        MetricFee2Partner,
        MetricFee2Usd,
        MetricFee3Advertiser,
        MetricFee3Partner,
        MetricFee3Usd,
        MetricFee4Advertiser,
        MetricFee4Partner,
        MetricFee4Usd,
        MetricFee5Advertiser,
        MetricFee5Partner,
        MetricFee5Usd,
        MetricFee6Advertiser,
        MetricFee6Partner,
        MetricFee6Usd,
        MetricFee7Advertiser,
        MetricFee7Partner,
        MetricFee7Usd,
        MetricFee8Advertiser,
        MetricFee8Partner,
        MetricFee8Usd,
        MetricFee9Advertiser,
        MetricFee9Partner,
        MetricFee9Usd,
        MetricFloodlightImpressions,
        MetricImpressions,
        MetricImpressionsToConversionRate,
        MetricLastClicks,
        MetricLastImpressions,
        MetricMediaCostAdvertiser,
        MetricMediaCostEcpapcAdvertiser,
        MetricMediaCostEcpapcPartner,
        MetricMediaCostEcpapcUsd,
        MetricMediaCostEcpapvAdvertiser,
        MetricMediaCostEcpapvPartner,
        MetricMediaCostEcpapvUsd,
        MetricMediaCostEcpaAdvertiser,
        MetricMediaCostEcpaPartner,
        MetricMediaCostEcpaUsd,
        MetricMediaCostEcpcvAdvertiser,
        MetricMediaCostEcpcvPartner,
        MetricMediaCostEcpcvUsd,
        MetricMediaCostEcpcAdvertiser,
        MetricMediaCostEcpcPartner,
        MetricMediaCostEcpcUsd,
        MetricMediaCostEcpmAdvertiser,
        MetricMediaCostEcpmPartner,
        MetricMediaCostEcpmUsd,
        MetricMediaCostPartner,
        MetricMediaCostUsd,
        MetricMediaCostViewableEcpmAdvertiser,
        MetricMediaCostViewableEcpmPartner,
        MetricMediaCostViewableEcpmUsd,
        MetricMediaFee1Advertiser,
        MetricMediaFee1Partner,
        MetricMediaFee1Usd,
        MetricMediaFee2Advertiser,
        MetricMediaFee2Partner,
        MetricMediaFee2Usd,
        MetricMediaFee3Advertiser,
        MetricMediaFee3Partner,
        MetricMediaFee3Usd,
        MetricMediaFee4Advertiser,
        MetricMediaFee4Partner,
        MetricMediaFee4Usd,
        MetricMediaFee5Advertiser,
        MetricMediaFee5Partner,
        MetricMediaFee5Usd,
        MetricPixelLoads,
        MetricPlatformFeeAdvertiser,
        MetricPlatformFeePartner,
        MetricPlatformFeeUsd,
        MetricPostClickDfaRevenue,
        MetricPostViewDfaRevenue,
        MetricProfitAdvertiser,
        MetricProfitEcpapcAdvertiser,
        MetricProfitEcpapcPartner,
        MetricProfitEcpapcUsd,
        MetricProfitEcpapvAdvertiser,
        MetricProfitEcpapvPartner,
        MetricProfitEcpapvUsd,
        MetricProfitEcpaAdvertiser,
        MetricProfitEcpaPartner,
        MetricProfitEcpaUsd,
        MetricProfitEcpcAdvertiser,
        MetricProfitEcpcPartner,
        MetricProfitEcpcUsd,
        MetricProfitEcpmAdvertiser,
        MetricProfitEcpmPartner,
        MetricProfitEcpmUsd,
        MetricProfitMargin,
        MetricProfitPartner,
        MetricProfitUsd,
        MetricProfitViewableEcpmAdvertiser,
        MetricProfitViewableEcpmPartner,
        MetricProfitViewableEcpmUsd,
        MetricReachCookieFrequency,
        MetricReachCookieReach,
        MetricRevenueAdvertiser,
        MetricRevenueEcpapcAdvertiser,
        MetricRevenueEcpapcPartner,
        MetricRevenueEcpapcUsd,
        MetricRevenueEcpapvAdvertiser,
        MetricRevenueEcpapvPartner,
        MetricRevenueEcpapvUsd,
        MetricRevenueEcpaAdvertiser,
        MetricRevenueEcpaPartner,
        MetricRevenueEcpaUsd,
        MetricRevenueEcpcvAdvertiser,
        MetricRevenueEcpcvPartner,
        MetricRevenueEcpcvUsd,
        MetricRevenueEcpcAdvertiser,
        MetricRevenueEcpcPartner,
        MetricRevenueEcpcUsd,
        MetricRevenueEcpiavcAdvertiser,
        MetricRevenueEcpmAdvertiser,
        MetricRevenueEcpmPartner,
        MetricRevenueEcpmUsd,
        MetricRevenuePartner,
        MetricRevenueUsd,
        MetricRevenueViewableEcpmAdvertiser,
        MetricRevenueViewableEcpmPartner,
        MetricRevenueViewableEcpmUsd,
        MetricRichMediaScrolls,
        MetricRichMediaVideoCompletions,
        MetricRichMediaVideoFirstQuartileCompletes,
        MetricRichMediaVideoFullScreens,
        MetricRichMediaVideoMidpoints,
        MetricRichMediaVideoMutes,
        MetricRichMediaVideoPauses,
        MetricRichMediaVideoPlays,
        MetricRichMediaVideoSkips,
        MetricRichMediaVideoThirdQuartileCompletes,
        MetricTeaTrueviewImpressions,
        MetricTeaTrueviewUniqueCookies,
        MetricTotalConversions,
        MetricTotalMediaCostAdvertiser,
        MetricTotalMediaCostEcpapcAdvertiser,
        MetricTotalMediaCostEcpapcPartner,
        MetricTotalMediaCostEcpapcUsd,
        MetricTotalMediaCostEcpapvAdvertiser,
        MetricTotalMediaCostEcpapvPartner,
        MetricTotalMediaCostEcpapvUsd,
        MetricTotalMediaCostEcpaAdvertiser,
        MetricTotalMediaCostEcpaPartner,
        MetricTotalMediaCostEcpaUsd,
        MetricTotalMediaCostEcpcvAdvertiser,
        MetricTotalMediaCostEcpcvPartner,
        MetricTotalMediaCostEcpcvUsd,
        MetricTotalMediaCostEcpcAdvertiser,
        MetricTotalMediaCostEcpcPartner,
        MetricTotalMediaCostEcpcUsd,
        MetricTotalMediaCostEcpmAdvertiser,
        MetricTotalMediaCostEcpmPartner,
        MetricTotalMediaCostEcpmUsd,
        MetricTotalMediaCostPartner,
        MetricTotalMediaCostUsd,
        MetricTotalMediaCostViewableEcpmAdvertiser,
        MetricTotalMediaCostViewableEcpmPartner,
        MetricTotalMediaCostViewableEcpmUsd,
        MetricTrueviewAverageCpeAdvertiser,
        MetricTrueviewAverageCpePartner,
        MetricTrueviewAverageCpeUsd,
        MetricTrueviewConversionCostManyPerViewAdvertiser,
        MetricTrueviewConversionCostManyPerViewPartner,
        MetricTrueviewConversionCostManyPerViewUsd,
        MetricTrueviewConversionCostOnePerViewAdvertiser,
        MetricTrueviewConversionCostOnePerViewPartner,
        MetricTrueviewConversionCostOnePerViewUsd,
        MetricTrueviewConversionManyPerView,
        MetricTrueviewConversionOnePerView,
        MetricTrueviewConversionRateOnePerView,
        MetricTrueviewConversionValueManyPerViewAdvertiser,
        MetricTrueviewConversionValueManyPerViewPartner,
        MetricTrueviewConversionValueManyPerViewUsd,
        MetricTrueviewConversionValueOnePerViewAdvertiser,
        MetricTrueviewConversionValueOnePerViewPartner,
        MetricTrueviewConversionValueOnePerViewUsd,
        MetricTrueviewCostConversionManyPerViewRatio,
        MetricTrueviewCostConversionOnePerViewRatio,
        MetricTrueviewCpvAdvertiser,
        MetricTrueviewCpvPartner,
        MetricTrueviewCpvUsd,
        MetricTrueviewEarnedLikes,
        MetricTrueviewEarnedPlaylistAdditions,
        MetricTrueviewEarnedShares,
        MetricTrueviewEarnedSubscribers,
        MetricTrueviewEarnedViews,
        MetricTrueviewEngagements,
        MetricTrueviewEngagementRate,
        MetricTrueviewImpressionShare,
        MetricTrueviewLostIsBudget,
        MetricTrueviewLostIsRank,
        MetricTrueviewTotalConversionValue,
        MetricTrueviewTotalConversionValuesAdvertiser,
        MetricTrueviewTotalConversionValuesPartner,
        MetricTrueviewTotalConversionValuesUsd,
        MetricTrueviewUniqueViewers,
        MetricTrueviewValueConversionManyPerViewRatio,
        MetricTrueviewValueConversionOnePerViewRatio,
        MetricTrueviewViews,
        MetricTrueviewViewRate,
        MetricTrueviewViewThroughConversion,
        MetricUniqueVisitorsCookies,
        MetricUnknown,
        MetricVideoCompanionClicks,
        MetricVideoCompanionImpressions,
        MetricVideoCompletionRate,
        MetricViewableBidRequests,
    }
    impl ParametersMetricsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ParametersMetricsItems::MetricActiveViewAudibleVisibleOnCompleteImpressions => {
                    "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewAverageViewableTime => {
                    "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME"
                }
                ParametersMetricsItems::MetricActiveViewDistributionUnmeasurable => {
                    "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE"
                }
                ParametersMetricsItems::MetricActiveViewDistributionUnviewable => {
                    "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE"
                }
                ParametersMetricsItems::MetricActiveViewDistributionViewable => {
                    "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE"
                }
                ParametersMetricsItems::MetricActiveViewEligibleImpressions => {
                    "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewMeasurableImpressions => {
                    "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewPctMeasurableImpressions => {
                    "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewPctViewableImpressions => {
                    "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleAtStart => {
                    "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START"
                }
                ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleFirstQuar => {
                    "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR"
                }
                ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleOnComplete => {
                    "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE"
                }
                ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleSecondQuar => {
                    "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR"
                }
                ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleThirdQuar => {
                    "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR"
                }
                ParametersMetricsItems::MetricActiveViewPercentViewableForTimeThreshold => {
                    "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD"
                }
                ParametersMetricsItems::MetricActiveViewPercentVisibleAtStart => {
                    "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START"
                }
                ParametersMetricsItems::MetricActiveViewPercentVisibleFirstQuar => {
                    "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR"
                }
                ParametersMetricsItems::MetricActiveViewPercentVisibleOnComplete => {
                    "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE"
                }
                ParametersMetricsItems::MetricActiveViewPercentVisibleSecondQuar => {
                    "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR"
                }
                ParametersMetricsItems::MetricActiveViewPercentVisibleThirdQuar => {
                    "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR"
                }
                ParametersMetricsItems::MetricActiveViewUnmeasurableImpressions => {
                    "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewUnviewableImpressions => {
                    "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricActiveViewViewableForTimeThreshold => {
                    "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD"
                }
                ParametersMetricsItems::MetricActiveViewViewableImpressions => {
                    "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricBidRequests => "METRIC_BID_REQUESTS",
                ParametersMetricsItems::MetricBillableCostAdvertiser => {
                    "METRIC_BILLABLE_COST_ADVERTISER"
                }
                ParametersMetricsItems::MetricBillableCostPartner => "METRIC_BILLABLE_COST_PARTNER",
                ParametersMetricsItems::MetricBillableCostUsd => "METRIC_BILLABLE_COST_USD",
                ParametersMetricsItems::MetricBillableImpressions => "METRIC_BILLABLE_IMPRESSIONS",
                ParametersMetricsItems::MetricClicks => "METRIC_CLICKS",
                ParametersMetricsItems::MetricClickToPostClickConversionRate => {
                    "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE"
                }
                ParametersMetricsItems::MetricCmPostClickRevenue => "METRIC_CM_POST_CLICK_REVENUE",
                ParametersMetricsItems::MetricCmPostViewRevenue => "METRIC_CM_POST_VIEW_REVENUE",
                ParametersMetricsItems::MetricComscoreVceAudienceAvgFrequency => {
                    "METRIC_COMSCORE_VCE_AUDIENCE_AVG_FREQUENCY"
                }
                ParametersMetricsItems::MetricComscoreVceAudienceImpressions => {
                    "METRIC_COMSCORE_VCE_AUDIENCE_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricComscoreVceAudienceImpressionsShare => {
                    "METRIC_COMSCORE_VCE_AUDIENCE_IMPRESSIONS_SHARE"
                }
                ParametersMetricsItems::MetricComscoreVceAudienceReachPct => {
                    "METRIC_COMSCORE_VCE_AUDIENCE_REACH_PCT"
                }
                ParametersMetricsItems::MetricComscoreVceAudienceSharePct => {
                    "METRIC_COMSCORE_VCE_AUDIENCE_SHARE_PCT"
                }
                ParametersMetricsItems::MetricComscoreVceGrossRatingPoints => {
                    "METRIC_COMSCORE_VCE_GROSS_RATING_POINTS"
                }
                ParametersMetricsItems::MetricComscoreVcePopulation => {
                    "METRIC_COMSCORE_VCE_POPULATION"
                }
                ParametersMetricsItems::MetricComscoreVceUniqueAudience => {
                    "METRIC_COMSCORE_VCE_UNIQUE_AUDIENCE"
                }
                ParametersMetricsItems::MetricConversionsPerMille => "METRIC_CONVERSIONS_PER_MILLE",
                ParametersMetricsItems::MetricCookieReachAverageImpressionFrequency => {
                    "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY"
                }
                ParametersMetricsItems::MetricCookieReachImpressionReach => {
                    "METRIC_COOKIE_REACH_IMPRESSION_REACH"
                }
                ParametersMetricsItems::MetricCpmFee1Advertiser => "METRIC_CPM_FEE1_ADVERTISER",
                ParametersMetricsItems::MetricCpmFee1Partner => "METRIC_CPM_FEE1_PARTNER",
                ParametersMetricsItems::MetricCpmFee1Usd => "METRIC_CPM_FEE1_USD",
                ParametersMetricsItems::MetricCpmFee2Advertiser => "METRIC_CPM_FEE2_ADVERTISER",
                ParametersMetricsItems::MetricCpmFee2Partner => "METRIC_CPM_FEE2_PARTNER",
                ParametersMetricsItems::MetricCpmFee2Usd => "METRIC_CPM_FEE2_USD",
                ParametersMetricsItems::MetricCpmFee3Advertiser => "METRIC_CPM_FEE3_ADVERTISER",
                ParametersMetricsItems::MetricCpmFee3Partner => "METRIC_CPM_FEE3_PARTNER",
                ParametersMetricsItems::MetricCpmFee3Usd => "METRIC_CPM_FEE3_USD",
                ParametersMetricsItems::MetricCpmFee4Advertiser => "METRIC_CPM_FEE4_ADVERTISER",
                ParametersMetricsItems::MetricCpmFee4Partner => "METRIC_CPM_FEE4_PARTNER",
                ParametersMetricsItems::MetricCpmFee4Usd => "METRIC_CPM_FEE4_USD",
                ParametersMetricsItems::MetricCpmFee5Advertiser => "METRIC_CPM_FEE5_ADVERTISER",
                ParametersMetricsItems::MetricCpmFee5Partner => "METRIC_CPM_FEE5_PARTNER",
                ParametersMetricsItems::MetricCpmFee5Usd => "METRIC_CPM_FEE5_USD",
                ParametersMetricsItems::MetricCtr => "METRIC_CTR",
                ParametersMetricsItems::MetricDataCostAdvertiser => "METRIC_DATA_COST_ADVERTISER",
                ParametersMetricsItems::MetricDataCostPartner => "METRIC_DATA_COST_PARTNER",
                ParametersMetricsItems::MetricDataCostUsd => "METRIC_DATA_COST_USD",
                ParametersMetricsItems::MetricDbmEngagementRate => "METRIC_DBM_ENGAGEMENT_RATE",
                ParametersMetricsItems::MetricFee10Advertiser => "METRIC_FEE10_ADVERTISER",
                ParametersMetricsItems::MetricFee10Partner => "METRIC_FEE10_PARTNER",
                ParametersMetricsItems::MetricFee10Usd => "METRIC_FEE10_USD",
                ParametersMetricsItems::MetricFee11Advertiser => "METRIC_FEE11_ADVERTISER",
                ParametersMetricsItems::MetricFee11Partner => "METRIC_FEE11_PARTNER",
                ParametersMetricsItems::MetricFee11Usd => "METRIC_FEE11_USD",
                ParametersMetricsItems::MetricFee12Advertiser => "METRIC_FEE12_ADVERTISER",
                ParametersMetricsItems::MetricFee12Partner => "METRIC_FEE12_PARTNER",
                ParametersMetricsItems::MetricFee12Usd => "METRIC_FEE12_USD",
                ParametersMetricsItems::MetricFee13Advertiser => "METRIC_FEE13_ADVERTISER",
                ParametersMetricsItems::MetricFee13Partner => "METRIC_FEE13_PARTNER",
                ParametersMetricsItems::MetricFee13Usd => "METRIC_FEE13_USD",
                ParametersMetricsItems::MetricFee14Advertiser => "METRIC_FEE14_ADVERTISER",
                ParametersMetricsItems::MetricFee14Partner => "METRIC_FEE14_PARTNER",
                ParametersMetricsItems::MetricFee14Usd => "METRIC_FEE14_USD",
                ParametersMetricsItems::MetricFee15Advertiser => "METRIC_FEE15_ADVERTISER",
                ParametersMetricsItems::MetricFee15Partner => "METRIC_FEE15_PARTNER",
                ParametersMetricsItems::MetricFee15Usd => "METRIC_FEE15_USD",
                ParametersMetricsItems::MetricFee16Advertiser => "METRIC_FEE16_ADVERTISER",
                ParametersMetricsItems::MetricFee16Partner => "METRIC_FEE16_PARTNER",
                ParametersMetricsItems::MetricFee16Usd => "METRIC_FEE16_USD",
                ParametersMetricsItems::MetricFee17Advertiser => "METRIC_FEE17_ADVERTISER",
                ParametersMetricsItems::MetricFee17Partner => "METRIC_FEE17_PARTNER",
                ParametersMetricsItems::MetricFee17Usd => "METRIC_FEE17_USD",
                ParametersMetricsItems::MetricFee18Advertiser => "METRIC_FEE18_ADVERTISER",
                ParametersMetricsItems::MetricFee18Partner => "METRIC_FEE18_PARTNER",
                ParametersMetricsItems::MetricFee18Usd => "METRIC_FEE18_USD",
                ParametersMetricsItems::MetricFee19Advertiser => "METRIC_FEE19_ADVERTISER",
                ParametersMetricsItems::MetricFee19Partner => "METRIC_FEE19_PARTNER",
                ParametersMetricsItems::MetricFee19Usd => "METRIC_FEE19_USD",
                ParametersMetricsItems::MetricFee20Advertiser => "METRIC_FEE20_ADVERTISER",
                ParametersMetricsItems::MetricFee20Partner => "METRIC_FEE20_PARTNER",
                ParametersMetricsItems::MetricFee20Usd => "METRIC_FEE20_USD",
                ParametersMetricsItems::MetricFee21Advertiser => "METRIC_FEE21_ADVERTISER",
                ParametersMetricsItems::MetricFee21Partner => "METRIC_FEE21_PARTNER",
                ParametersMetricsItems::MetricFee21Usd => "METRIC_FEE21_USD",
                ParametersMetricsItems::MetricFee22Advertiser => "METRIC_FEE22_ADVERTISER",
                ParametersMetricsItems::MetricFee22Partner => "METRIC_FEE22_PARTNER",
                ParametersMetricsItems::MetricFee22Usd => "METRIC_FEE22_USD",
                ParametersMetricsItems::MetricFee2Advertiser => "METRIC_FEE2_ADVERTISER",
                ParametersMetricsItems::MetricFee2Partner => "METRIC_FEE2_PARTNER",
                ParametersMetricsItems::MetricFee2Usd => "METRIC_FEE2_USD",
                ParametersMetricsItems::MetricFee3Advertiser => "METRIC_FEE3_ADVERTISER",
                ParametersMetricsItems::MetricFee3Partner => "METRIC_FEE3_PARTNER",
                ParametersMetricsItems::MetricFee3Usd => "METRIC_FEE3_USD",
                ParametersMetricsItems::MetricFee4Advertiser => "METRIC_FEE4_ADVERTISER",
                ParametersMetricsItems::MetricFee4Partner => "METRIC_FEE4_PARTNER",
                ParametersMetricsItems::MetricFee4Usd => "METRIC_FEE4_USD",
                ParametersMetricsItems::MetricFee5Advertiser => "METRIC_FEE5_ADVERTISER",
                ParametersMetricsItems::MetricFee5Partner => "METRIC_FEE5_PARTNER",
                ParametersMetricsItems::MetricFee5Usd => "METRIC_FEE5_USD",
                ParametersMetricsItems::MetricFee6Advertiser => "METRIC_FEE6_ADVERTISER",
                ParametersMetricsItems::MetricFee6Partner => "METRIC_FEE6_PARTNER",
                ParametersMetricsItems::MetricFee6Usd => "METRIC_FEE6_USD",
                ParametersMetricsItems::MetricFee7Advertiser => "METRIC_FEE7_ADVERTISER",
                ParametersMetricsItems::MetricFee7Partner => "METRIC_FEE7_PARTNER",
                ParametersMetricsItems::MetricFee7Usd => "METRIC_FEE7_USD",
                ParametersMetricsItems::MetricFee8Advertiser => "METRIC_FEE8_ADVERTISER",
                ParametersMetricsItems::MetricFee8Partner => "METRIC_FEE8_PARTNER",
                ParametersMetricsItems::MetricFee8Usd => "METRIC_FEE8_USD",
                ParametersMetricsItems::MetricFee9Advertiser => "METRIC_FEE9_ADVERTISER",
                ParametersMetricsItems::MetricFee9Partner => "METRIC_FEE9_PARTNER",
                ParametersMetricsItems::MetricFee9Usd => "METRIC_FEE9_USD",
                ParametersMetricsItems::MetricFloodlightImpressions => {
                    "METRIC_FLOODLIGHT_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricImpressions => "METRIC_IMPRESSIONS",
                ParametersMetricsItems::MetricImpressionsToConversionRate => {
                    "METRIC_IMPRESSIONS_TO_CONVERSION_RATE"
                }
                ParametersMetricsItems::MetricLastClicks => "METRIC_LAST_CLICKS",
                ParametersMetricsItems::MetricLastImpressions => "METRIC_LAST_IMPRESSIONS",
                ParametersMetricsItems::MetricMediaCostAdvertiser => "METRIC_MEDIA_COST_ADVERTISER",
                ParametersMetricsItems::MetricMediaCostEcpapcAdvertiser => {
                    "METRIC_MEDIA_COST_ECPAPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostEcpapcPartner => {
                    "METRIC_MEDIA_COST_ECPAPC_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostEcpapcUsd => "METRIC_MEDIA_COST_ECPAPC_USD",
                ParametersMetricsItems::MetricMediaCostEcpapvAdvertiser => {
                    "METRIC_MEDIA_COST_ECPAPV_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostEcpapvPartner => {
                    "METRIC_MEDIA_COST_ECPAPV_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostEcpapvUsd => "METRIC_MEDIA_COST_ECPAPV_USD",
                ParametersMetricsItems::MetricMediaCostEcpaAdvertiser => {
                    "METRIC_MEDIA_COST_ECPA_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostEcpaPartner => {
                    "METRIC_MEDIA_COST_ECPA_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostEcpaUsd => "METRIC_MEDIA_COST_ECPA_USD",
                ParametersMetricsItems::MetricMediaCostEcpcvAdvertiser => {
                    "METRIC_MEDIA_COST_ECPCV_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostEcpcvPartner => {
                    "METRIC_MEDIA_COST_ECPCV_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostEcpcvUsd => "METRIC_MEDIA_COST_ECPCV_USD",
                ParametersMetricsItems::MetricMediaCostEcpcAdvertiser => {
                    "METRIC_MEDIA_COST_ECPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostEcpcPartner => {
                    "METRIC_MEDIA_COST_ECPC_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostEcpcUsd => "METRIC_MEDIA_COST_ECPC_USD",
                ParametersMetricsItems::MetricMediaCostEcpmAdvertiser => {
                    "METRIC_MEDIA_COST_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostEcpmPartner => {
                    "METRIC_MEDIA_COST_ECPM_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostEcpmUsd => "METRIC_MEDIA_COST_ECPM_USD",
                ParametersMetricsItems::MetricMediaCostPartner => "METRIC_MEDIA_COST_PARTNER",
                ParametersMetricsItems::MetricMediaCostUsd => "METRIC_MEDIA_COST_USD",
                ParametersMetricsItems::MetricMediaCostViewableEcpmAdvertiser => {
                    "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricMediaCostViewableEcpmPartner => {
                    "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER"
                }
                ParametersMetricsItems::MetricMediaCostViewableEcpmUsd => {
                    "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD"
                }
                ParametersMetricsItems::MetricMediaFee1Advertiser => "METRIC_MEDIA_FEE1_ADVERTISER",
                ParametersMetricsItems::MetricMediaFee1Partner => "METRIC_MEDIA_FEE1_PARTNER",
                ParametersMetricsItems::MetricMediaFee1Usd => "METRIC_MEDIA_FEE1_USD",
                ParametersMetricsItems::MetricMediaFee2Advertiser => "METRIC_MEDIA_FEE2_ADVERTISER",
                ParametersMetricsItems::MetricMediaFee2Partner => "METRIC_MEDIA_FEE2_PARTNER",
                ParametersMetricsItems::MetricMediaFee2Usd => "METRIC_MEDIA_FEE2_USD",
                ParametersMetricsItems::MetricMediaFee3Advertiser => "METRIC_MEDIA_FEE3_ADVERTISER",
                ParametersMetricsItems::MetricMediaFee3Partner => "METRIC_MEDIA_FEE3_PARTNER",
                ParametersMetricsItems::MetricMediaFee3Usd => "METRIC_MEDIA_FEE3_USD",
                ParametersMetricsItems::MetricMediaFee4Advertiser => "METRIC_MEDIA_FEE4_ADVERTISER",
                ParametersMetricsItems::MetricMediaFee4Partner => "METRIC_MEDIA_FEE4_PARTNER",
                ParametersMetricsItems::MetricMediaFee4Usd => "METRIC_MEDIA_FEE4_USD",
                ParametersMetricsItems::MetricMediaFee5Advertiser => "METRIC_MEDIA_FEE5_ADVERTISER",
                ParametersMetricsItems::MetricMediaFee5Partner => "METRIC_MEDIA_FEE5_PARTNER",
                ParametersMetricsItems::MetricMediaFee5Usd => "METRIC_MEDIA_FEE5_USD",
                ParametersMetricsItems::MetricPixelLoads => "METRIC_PIXEL_LOADS",
                ParametersMetricsItems::MetricPlatformFeeAdvertiser => {
                    "METRIC_PLATFORM_FEE_ADVERTISER"
                }
                ParametersMetricsItems::MetricPlatformFeePartner => "METRIC_PLATFORM_FEE_PARTNER",
                ParametersMetricsItems::MetricPlatformFeeUsd => "METRIC_PLATFORM_FEE_USD",
                ParametersMetricsItems::MetricPostClickDfaRevenue => {
                    "METRIC_POST_CLICK_DFA_REVENUE"
                }
                ParametersMetricsItems::MetricPostViewDfaRevenue => "METRIC_POST_VIEW_DFA_REVENUE",
                ParametersMetricsItems::MetricProfitAdvertiser => "METRIC_PROFIT_ADVERTISER",
                ParametersMetricsItems::MetricProfitEcpapcAdvertiser => {
                    "METRIC_PROFIT_ECPAPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricProfitEcpapcPartner => "METRIC_PROFIT_ECPAPC_PARTNER",
                ParametersMetricsItems::MetricProfitEcpapcUsd => "METRIC_PROFIT_ECPAPC_USD",
                ParametersMetricsItems::MetricProfitEcpapvAdvertiser => {
                    "METRIC_PROFIT_ECPAPV_ADVERTISER"
                }
                ParametersMetricsItems::MetricProfitEcpapvPartner => "METRIC_PROFIT_ECPAPV_PARTNER",
                ParametersMetricsItems::MetricProfitEcpapvUsd => "METRIC_PROFIT_ECPAPV_USD",
                ParametersMetricsItems::MetricProfitEcpaAdvertiser => {
                    "METRIC_PROFIT_ECPA_ADVERTISER"
                }
                ParametersMetricsItems::MetricProfitEcpaPartner => "METRIC_PROFIT_ECPA_PARTNER",
                ParametersMetricsItems::MetricProfitEcpaUsd => "METRIC_PROFIT_ECPA_USD",
                ParametersMetricsItems::MetricProfitEcpcAdvertiser => {
                    "METRIC_PROFIT_ECPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricProfitEcpcPartner => "METRIC_PROFIT_ECPC_PARTNER",
                ParametersMetricsItems::MetricProfitEcpcUsd => "METRIC_PROFIT_ECPC_USD",
                ParametersMetricsItems::MetricProfitEcpmAdvertiser => {
                    "METRIC_PROFIT_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricProfitEcpmPartner => "METRIC_PROFIT_ECPM_PARTNER",
                ParametersMetricsItems::MetricProfitEcpmUsd => "METRIC_PROFIT_ECPM_USD",
                ParametersMetricsItems::MetricProfitMargin => "METRIC_PROFIT_MARGIN",
                ParametersMetricsItems::MetricProfitPartner => "METRIC_PROFIT_PARTNER",
                ParametersMetricsItems::MetricProfitUsd => "METRIC_PROFIT_USD",
                ParametersMetricsItems::MetricProfitViewableEcpmAdvertiser => {
                    "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricProfitViewableEcpmPartner => {
                    "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER"
                }
                ParametersMetricsItems::MetricProfitViewableEcpmUsd => {
                    "METRIC_PROFIT_VIEWABLE_ECPM_USD"
                }
                ParametersMetricsItems::MetricReachCookieFrequency => {
                    "METRIC_REACH_COOKIE_FREQUENCY"
                }
                ParametersMetricsItems::MetricReachCookieReach => "METRIC_REACH_COOKIE_REACH",
                ParametersMetricsItems::MetricRevenueAdvertiser => "METRIC_REVENUE_ADVERTISER",
                ParametersMetricsItems::MetricRevenueEcpapcAdvertiser => {
                    "METRIC_REVENUE_ECPAPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpapcPartner => {
                    "METRIC_REVENUE_ECPAPC_PARTNER"
                }
                ParametersMetricsItems::MetricRevenueEcpapcUsd => "METRIC_REVENUE_ECPAPC_USD",
                ParametersMetricsItems::MetricRevenueEcpapvAdvertiser => {
                    "METRIC_REVENUE_ECPAPV_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpapvPartner => {
                    "METRIC_REVENUE_ECPAPV_PARTNER"
                }
                ParametersMetricsItems::MetricRevenueEcpapvUsd => "METRIC_REVENUE_ECPAPV_USD",
                ParametersMetricsItems::MetricRevenueEcpaAdvertiser => {
                    "METRIC_REVENUE_ECPA_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpaPartner => "METRIC_REVENUE_ECPA_PARTNER",
                ParametersMetricsItems::MetricRevenueEcpaUsd => "METRIC_REVENUE_ECPA_USD",
                ParametersMetricsItems::MetricRevenueEcpcvAdvertiser => {
                    "METRIC_REVENUE_ECPCV_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpcvPartner => "METRIC_REVENUE_ECPCV_PARTNER",
                ParametersMetricsItems::MetricRevenueEcpcvUsd => "METRIC_REVENUE_ECPCV_USD",
                ParametersMetricsItems::MetricRevenueEcpcAdvertiser => {
                    "METRIC_REVENUE_ECPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpcPartner => "METRIC_REVENUE_ECPC_PARTNER",
                ParametersMetricsItems::MetricRevenueEcpcUsd => "METRIC_REVENUE_ECPC_USD",
                ParametersMetricsItems::MetricRevenueEcpiavcAdvertiser => {
                    "METRIC_REVENUE_ECPIAVC_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpmAdvertiser => {
                    "METRIC_REVENUE_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueEcpmPartner => "METRIC_REVENUE_ECPM_PARTNER",
                ParametersMetricsItems::MetricRevenueEcpmUsd => "METRIC_REVENUE_ECPM_USD",
                ParametersMetricsItems::MetricRevenuePartner => "METRIC_REVENUE_PARTNER",
                ParametersMetricsItems::MetricRevenueUsd => "METRIC_REVENUE_USD",
                ParametersMetricsItems::MetricRevenueViewableEcpmAdvertiser => {
                    "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricRevenueViewableEcpmPartner => {
                    "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER"
                }
                ParametersMetricsItems::MetricRevenueViewableEcpmUsd => {
                    "METRIC_REVENUE_VIEWABLE_ECPM_USD"
                }
                ParametersMetricsItems::MetricRichMediaScrolls => "METRIC_RICH_MEDIA_SCROLLS",
                ParametersMetricsItems::MetricRichMediaVideoCompletions => {
                    "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS"
                }
                ParametersMetricsItems::MetricRichMediaVideoFirstQuartileCompletes => {
                    "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES"
                }
                ParametersMetricsItems::MetricRichMediaVideoFullScreens => {
                    "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS"
                }
                ParametersMetricsItems::MetricRichMediaVideoMidpoints => {
                    "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS"
                }
                ParametersMetricsItems::MetricRichMediaVideoMutes => {
                    "METRIC_RICH_MEDIA_VIDEO_MUTES"
                }
                ParametersMetricsItems::MetricRichMediaVideoPauses => {
                    "METRIC_RICH_MEDIA_VIDEO_PAUSES"
                }
                ParametersMetricsItems::MetricRichMediaVideoPlays => {
                    "METRIC_RICH_MEDIA_VIDEO_PLAYS"
                }
                ParametersMetricsItems::MetricRichMediaVideoSkips => {
                    "METRIC_RICH_MEDIA_VIDEO_SKIPS"
                }
                ParametersMetricsItems::MetricRichMediaVideoThirdQuartileCompletes => {
                    "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES"
                }
                ParametersMetricsItems::MetricTeaTrueviewImpressions => {
                    "METRIC_TEA_TRUEVIEW_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricTeaTrueviewUniqueCookies => {
                    "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES"
                }
                ParametersMetricsItems::MetricTotalConversions => "METRIC_TOTAL_CONVERSIONS",
                ParametersMetricsItems::MetricTotalMediaCostAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpapcAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpapcPartner => {
                    "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpapcUsd => {
                    "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpapvAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpapvPartner => {
                    "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpapvUsd => {
                    "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpaAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpaPartner => {
                    "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpaUsd => {
                    "METRIC_TOTAL_MEDIA_COST_ECPA_USD"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpcvAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpcvPartner => {
                    "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpcvUsd => {
                    "METRIC_TOTAL_MEDIA_COST_ECPCV_USD"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpcAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpcPartner => {
                    "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpcUsd => {
                    "METRIC_TOTAL_MEDIA_COST_ECPC_USD"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpmAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpmPartner => {
                    "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostEcpmUsd => {
                    "METRIC_TOTAL_MEDIA_COST_ECPM_USD"
                }
                ParametersMetricsItems::MetricTotalMediaCostPartner => {
                    "METRIC_TOTAL_MEDIA_COST_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostUsd => "METRIC_TOTAL_MEDIA_COST_USD",
                ParametersMetricsItems::MetricTotalMediaCostViewableEcpmAdvertiser => {
                    "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER"
                }
                ParametersMetricsItems::MetricTotalMediaCostViewableEcpmPartner => {
                    "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER"
                }
                ParametersMetricsItems::MetricTotalMediaCostViewableEcpmUsd => {
                    "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD"
                }
                ParametersMetricsItems::MetricTrueviewAverageCpeAdvertiser => {
                    "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewAverageCpePartner => {
                    "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER"
                }
                ParametersMetricsItems::MetricTrueviewAverageCpeUsd => {
                    "METRIC_TRUEVIEW_AVERAGE_CPE_USD"
                }
                ParametersMetricsItems::MetricTrueviewConversionCostManyPerViewAdvertiser => {
                    "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewConversionCostManyPerViewPartner => {
                    "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER"
                }
                ParametersMetricsItems::MetricTrueviewConversionCostManyPerViewUsd => {
                    "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD"
                }
                ParametersMetricsItems::MetricTrueviewConversionCostOnePerViewAdvertiser => {
                    "METRIC_TRUEVIEW_CONVERSION_COST_ONE_PER_VIEW_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewConversionCostOnePerViewPartner => {
                    "METRIC_TRUEVIEW_CONVERSION_COST_ONE_PER_VIEW_PARTNER"
                }
                ParametersMetricsItems::MetricTrueviewConversionCostOnePerViewUsd => {
                    "METRIC_TRUEVIEW_CONVERSION_COST_ONE_PER_VIEW_USD"
                }
                ParametersMetricsItems::MetricTrueviewConversionManyPerView => {
                    "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW"
                }
                ParametersMetricsItems::MetricTrueviewConversionOnePerView => {
                    "METRIC_TRUEVIEW_CONVERSION_ONE_PER_VIEW"
                }
                ParametersMetricsItems::MetricTrueviewConversionRateOnePerView => {
                    "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW"
                }
                ParametersMetricsItems::MetricTrueviewConversionValueManyPerViewAdvertiser => {
                    "METRIC_TRUEVIEW_CONVERSION_VALUE_MANY_PER_VIEW_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewConversionValueManyPerViewPartner => {
                    "METRIC_TRUEVIEW_CONVERSION_VALUE_MANY_PER_VIEW_PARTNER"
                }
                ParametersMetricsItems::MetricTrueviewConversionValueManyPerViewUsd => {
                    "METRIC_TRUEVIEW_CONVERSION_VALUE_MANY_PER_VIEW_USD"
                }
                ParametersMetricsItems::MetricTrueviewConversionValueOnePerViewAdvertiser => {
                    "METRIC_TRUEVIEW_CONVERSION_VALUE_ONE_PER_VIEW_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewConversionValueOnePerViewPartner => {
                    "METRIC_TRUEVIEW_CONVERSION_VALUE_ONE_PER_VIEW_PARTNER"
                }
                ParametersMetricsItems::MetricTrueviewConversionValueOnePerViewUsd => {
                    "METRIC_TRUEVIEW_CONVERSION_VALUE_ONE_PER_VIEW_USD"
                }
                ParametersMetricsItems::MetricTrueviewCostConversionManyPerViewRatio => {
                    "METRIC_TRUEVIEW_COST_CONVERSION_MANY_PER_VIEW_RATIO"
                }
                ParametersMetricsItems::MetricTrueviewCostConversionOnePerViewRatio => {
                    "METRIC_TRUEVIEW_COST_CONVERSION_ONE_PER_VIEW_RATIO"
                }
                ParametersMetricsItems::MetricTrueviewCpvAdvertiser => {
                    "METRIC_TRUEVIEW_CPV_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewCpvPartner => "METRIC_TRUEVIEW_CPV_PARTNER",
                ParametersMetricsItems::MetricTrueviewCpvUsd => "METRIC_TRUEVIEW_CPV_USD",
                ParametersMetricsItems::MetricTrueviewEarnedLikes => "METRIC_TRUEVIEW_EARNED_LIKES",
                ParametersMetricsItems::MetricTrueviewEarnedPlaylistAdditions => {
                    "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS"
                }
                ParametersMetricsItems::MetricTrueviewEarnedShares => {
                    "METRIC_TRUEVIEW_EARNED_SHARES"
                }
                ParametersMetricsItems::MetricTrueviewEarnedSubscribers => {
                    "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS"
                }
                ParametersMetricsItems::MetricTrueviewEarnedViews => "METRIC_TRUEVIEW_EARNED_VIEWS",
                ParametersMetricsItems::MetricTrueviewEngagements => "METRIC_TRUEVIEW_ENGAGEMENTS",
                ParametersMetricsItems::MetricTrueviewEngagementRate => {
                    "METRIC_TRUEVIEW_ENGAGEMENT_RATE"
                }
                ParametersMetricsItems::MetricTrueviewImpressionShare => {
                    "METRIC_TRUEVIEW_IMPRESSION_SHARE"
                }
                ParametersMetricsItems::MetricTrueviewLostIsBudget => {
                    "METRIC_TRUEVIEW_LOST_IS_BUDGET"
                }
                ParametersMetricsItems::MetricTrueviewLostIsRank => "METRIC_TRUEVIEW_LOST_IS_RANK",
                ParametersMetricsItems::MetricTrueviewTotalConversionValue => {
                    "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUE"
                }
                ParametersMetricsItems::MetricTrueviewTotalConversionValuesAdvertiser => {
                    "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER"
                }
                ParametersMetricsItems::MetricTrueviewTotalConversionValuesPartner => {
                    "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER"
                }
                ParametersMetricsItems::MetricTrueviewTotalConversionValuesUsd => {
                    "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD"
                }
                ParametersMetricsItems::MetricTrueviewUniqueViewers => {
                    "METRIC_TRUEVIEW_UNIQUE_VIEWERS"
                }
                ParametersMetricsItems::MetricTrueviewValueConversionManyPerViewRatio => {
                    "METRIC_TRUEVIEW_VALUE_CONVERSION_MANY_PER_VIEW_RATIO"
                }
                ParametersMetricsItems::MetricTrueviewValueConversionOnePerViewRatio => {
                    "METRIC_TRUEVIEW_VALUE_CONVERSION_ONE_PER_VIEW_RATIO"
                }
                ParametersMetricsItems::MetricTrueviewViews => "METRIC_TRUEVIEW_VIEWS",
                ParametersMetricsItems::MetricTrueviewViewRate => "METRIC_TRUEVIEW_VIEW_RATE",
                ParametersMetricsItems::MetricTrueviewViewThroughConversion => {
                    "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION"
                }
                ParametersMetricsItems::MetricUniqueVisitorsCookies => {
                    "METRIC_UNIQUE_VISITORS_COOKIES"
                }
                ParametersMetricsItems::MetricUnknown => "METRIC_UNKNOWN",
                ParametersMetricsItems::MetricVideoCompanionClicks => {
                    "METRIC_VIDEO_COMPANION_CLICKS"
                }
                ParametersMetricsItems::MetricVideoCompanionImpressions => {
                    "METRIC_VIDEO_COMPANION_IMPRESSIONS"
                }
                ParametersMetricsItems::MetricVideoCompletionRate => "METRIC_VIDEO_COMPLETION_RATE",
                ParametersMetricsItems::MetricViewableBidRequests => "METRIC_VIEWABLE_BID_REQUESTS",
            }
        }
    }
    impl ::std::fmt::Display for ParametersMetricsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParametersMetricsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParametersMetricsItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewAudibleVisibleOnCompleteImpressions
                }
                "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME" => {
                    ParametersMetricsItems::MetricActiveViewAverageViewableTime
                }
                "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE" => {
                    ParametersMetricsItems::MetricActiveViewDistributionUnmeasurable
                }
                "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE" => {
                    ParametersMetricsItems::MetricActiveViewDistributionUnviewable
                }
                "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE" => {
                    ParametersMetricsItems::MetricActiveViewDistributionViewable
                }
                "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewEligibleImpressions
                }
                "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewMeasurableImpressions
                }
                "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewPctMeasurableImpressions
                }
                "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewPctViewableImpressions
                }
                "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START" => {
                    ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleAtStart
                }
                "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR" => {
                    ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleFirstQuar
                }
                "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE" => {
                    ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleOnComplete
                }
                "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR" => {
                    ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleSecondQuar
                }
                "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR" => {
                    ParametersMetricsItems::MetricActiveViewPercentAudibleVisibleThirdQuar
                }
                "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD" => {
                    ParametersMetricsItems::MetricActiveViewPercentViewableForTimeThreshold
                }
                "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START" => {
                    ParametersMetricsItems::MetricActiveViewPercentVisibleAtStart
                }
                "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR" => {
                    ParametersMetricsItems::MetricActiveViewPercentVisibleFirstQuar
                }
                "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE" => {
                    ParametersMetricsItems::MetricActiveViewPercentVisibleOnComplete
                }
                "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR" => {
                    ParametersMetricsItems::MetricActiveViewPercentVisibleSecondQuar
                }
                "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR" => {
                    ParametersMetricsItems::MetricActiveViewPercentVisibleThirdQuar
                }
                "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewUnmeasurableImpressions
                }
                "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewUnviewableImpressions
                }
                "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD" => {
                    ParametersMetricsItems::MetricActiveViewViewableForTimeThreshold
                }
                "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricActiveViewViewableImpressions
                }
                "METRIC_BID_REQUESTS" => ParametersMetricsItems::MetricBidRequests,
                "METRIC_BILLABLE_COST_ADVERTISER" => {
                    ParametersMetricsItems::MetricBillableCostAdvertiser
                }
                "METRIC_BILLABLE_COST_PARTNER" => ParametersMetricsItems::MetricBillableCostPartner,
                "METRIC_BILLABLE_COST_USD" => ParametersMetricsItems::MetricBillableCostUsd,
                "METRIC_BILLABLE_IMPRESSIONS" => ParametersMetricsItems::MetricBillableImpressions,
                "METRIC_CLICKS" => ParametersMetricsItems::MetricClicks,
                "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE" => {
                    ParametersMetricsItems::MetricClickToPostClickConversionRate
                }
                "METRIC_CM_POST_CLICK_REVENUE" => ParametersMetricsItems::MetricCmPostClickRevenue,
                "METRIC_CM_POST_VIEW_REVENUE" => ParametersMetricsItems::MetricCmPostViewRevenue,
                "METRIC_COMSCORE_VCE_AUDIENCE_AVG_FREQUENCY" => {
                    ParametersMetricsItems::MetricComscoreVceAudienceAvgFrequency
                }
                "METRIC_COMSCORE_VCE_AUDIENCE_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricComscoreVceAudienceImpressions
                }
                "METRIC_COMSCORE_VCE_AUDIENCE_IMPRESSIONS_SHARE" => {
                    ParametersMetricsItems::MetricComscoreVceAudienceImpressionsShare
                }
                "METRIC_COMSCORE_VCE_AUDIENCE_REACH_PCT" => {
                    ParametersMetricsItems::MetricComscoreVceAudienceReachPct
                }
                "METRIC_COMSCORE_VCE_AUDIENCE_SHARE_PCT" => {
                    ParametersMetricsItems::MetricComscoreVceAudienceSharePct
                }
                "METRIC_COMSCORE_VCE_GROSS_RATING_POINTS" => {
                    ParametersMetricsItems::MetricComscoreVceGrossRatingPoints
                }
                "METRIC_COMSCORE_VCE_POPULATION" => {
                    ParametersMetricsItems::MetricComscoreVcePopulation
                }
                "METRIC_COMSCORE_VCE_UNIQUE_AUDIENCE" => {
                    ParametersMetricsItems::MetricComscoreVceUniqueAudience
                }
                "METRIC_CONVERSIONS_PER_MILLE" => ParametersMetricsItems::MetricConversionsPerMille,
                "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY" => {
                    ParametersMetricsItems::MetricCookieReachAverageImpressionFrequency
                }
                "METRIC_COOKIE_REACH_IMPRESSION_REACH" => {
                    ParametersMetricsItems::MetricCookieReachImpressionReach
                }
                "METRIC_CPM_FEE1_ADVERTISER" => ParametersMetricsItems::MetricCpmFee1Advertiser,
                "METRIC_CPM_FEE1_PARTNER" => ParametersMetricsItems::MetricCpmFee1Partner,
                "METRIC_CPM_FEE1_USD" => ParametersMetricsItems::MetricCpmFee1Usd,
                "METRIC_CPM_FEE2_ADVERTISER" => ParametersMetricsItems::MetricCpmFee2Advertiser,
                "METRIC_CPM_FEE2_PARTNER" => ParametersMetricsItems::MetricCpmFee2Partner,
                "METRIC_CPM_FEE2_USD" => ParametersMetricsItems::MetricCpmFee2Usd,
                "METRIC_CPM_FEE3_ADVERTISER" => ParametersMetricsItems::MetricCpmFee3Advertiser,
                "METRIC_CPM_FEE3_PARTNER" => ParametersMetricsItems::MetricCpmFee3Partner,
                "METRIC_CPM_FEE3_USD" => ParametersMetricsItems::MetricCpmFee3Usd,
                "METRIC_CPM_FEE4_ADVERTISER" => ParametersMetricsItems::MetricCpmFee4Advertiser,
                "METRIC_CPM_FEE4_PARTNER" => ParametersMetricsItems::MetricCpmFee4Partner,
                "METRIC_CPM_FEE4_USD" => ParametersMetricsItems::MetricCpmFee4Usd,
                "METRIC_CPM_FEE5_ADVERTISER" => ParametersMetricsItems::MetricCpmFee5Advertiser,
                "METRIC_CPM_FEE5_PARTNER" => ParametersMetricsItems::MetricCpmFee5Partner,
                "METRIC_CPM_FEE5_USD" => ParametersMetricsItems::MetricCpmFee5Usd,
                "METRIC_CTR" => ParametersMetricsItems::MetricCtr,
                "METRIC_DATA_COST_ADVERTISER" => ParametersMetricsItems::MetricDataCostAdvertiser,
                "METRIC_DATA_COST_PARTNER" => ParametersMetricsItems::MetricDataCostPartner,
                "METRIC_DATA_COST_USD" => ParametersMetricsItems::MetricDataCostUsd,
                "METRIC_DBM_ENGAGEMENT_RATE" => ParametersMetricsItems::MetricDbmEngagementRate,
                "METRIC_FEE10_ADVERTISER" => ParametersMetricsItems::MetricFee10Advertiser,
                "METRIC_FEE10_PARTNER" => ParametersMetricsItems::MetricFee10Partner,
                "METRIC_FEE10_USD" => ParametersMetricsItems::MetricFee10Usd,
                "METRIC_FEE11_ADVERTISER" => ParametersMetricsItems::MetricFee11Advertiser,
                "METRIC_FEE11_PARTNER" => ParametersMetricsItems::MetricFee11Partner,
                "METRIC_FEE11_USD" => ParametersMetricsItems::MetricFee11Usd,
                "METRIC_FEE12_ADVERTISER" => ParametersMetricsItems::MetricFee12Advertiser,
                "METRIC_FEE12_PARTNER" => ParametersMetricsItems::MetricFee12Partner,
                "METRIC_FEE12_USD" => ParametersMetricsItems::MetricFee12Usd,
                "METRIC_FEE13_ADVERTISER" => ParametersMetricsItems::MetricFee13Advertiser,
                "METRIC_FEE13_PARTNER" => ParametersMetricsItems::MetricFee13Partner,
                "METRIC_FEE13_USD" => ParametersMetricsItems::MetricFee13Usd,
                "METRIC_FEE14_ADVERTISER" => ParametersMetricsItems::MetricFee14Advertiser,
                "METRIC_FEE14_PARTNER" => ParametersMetricsItems::MetricFee14Partner,
                "METRIC_FEE14_USD" => ParametersMetricsItems::MetricFee14Usd,
                "METRIC_FEE15_ADVERTISER" => ParametersMetricsItems::MetricFee15Advertiser,
                "METRIC_FEE15_PARTNER" => ParametersMetricsItems::MetricFee15Partner,
                "METRIC_FEE15_USD" => ParametersMetricsItems::MetricFee15Usd,
                "METRIC_FEE16_ADVERTISER" => ParametersMetricsItems::MetricFee16Advertiser,
                "METRIC_FEE16_PARTNER" => ParametersMetricsItems::MetricFee16Partner,
                "METRIC_FEE16_USD" => ParametersMetricsItems::MetricFee16Usd,
                "METRIC_FEE17_ADVERTISER" => ParametersMetricsItems::MetricFee17Advertiser,
                "METRIC_FEE17_PARTNER" => ParametersMetricsItems::MetricFee17Partner,
                "METRIC_FEE17_USD" => ParametersMetricsItems::MetricFee17Usd,
                "METRIC_FEE18_ADVERTISER" => ParametersMetricsItems::MetricFee18Advertiser,
                "METRIC_FEE18_PARTNER" => ParametersMetricsItems::MetricFee18Partner,
                "METRIC_FEE18_USD" => ParametersMetricsItems::MetricFee18Usd,
                "METRIC_FEE19_ADVERTISER" => ParametersMetricsItems::MetricFee19Advertiser,
                "METRIC_FEE19_PARTNER" => ParametersMetricsItems::MetricFee19Partner,
                "METRIC_FEE19_USD" => ParametersMetricsItems::MetricFee19Usd,
                "METRIC_FEE20_ADVERTISER" => ParametersMetricsItems::MetricFee20Advertiser,
                "METRIC_FEE20_PARTNER" => ParametersMetricsItems::MetricFee20Partner,
                "METRIC_FEE20_USD" => ParametersMetricsItems::MetricFee20Usd,
                "METRIC_FEE21_ADVERTISER" => ParametersMetricsItems::MetricFee21Advertiser,
                "METRIC_FEE21_PARTNER" => ParametersMetricsItems::MetricFee21Partner,
                "METRIC_FEE21_USD" => ParametersMetricsItems::MetricFee21Usd,
                "METRIC_FEE22_ADVERTISER" => ParametersMetricsItems::MetricFee22Advertiser,
                "METRIC_FEE22_PARTNER" => ParametersMetricsItems::MetricFee22Partner,
                "METRIC_FEE22_USD" => ParametersMetricsItems::MetricFee22Usd,
                "METRIC_FEE2_ADVERTISER" => ParametersMetricsItems::MetricFee2Advertiser,
                "METRIC_FEE2_PARTNER" => ParametersMetricsItems::MetricFee2Partner,
                "METRIC_FEE2_USD" => ParametersMetricsItems::MetricFee2Usd,
                "METRIC_FEE3_ADVERTISER" => ParametersMetricsItems::MetricFee3Advertiser,
                "METRIC_FEE3_PARTNER" => ParametersMetricsItems::MetricFee3Partner,
                "METRIC_FEE3_USD" => ParametersMetricsItems::MetricFee3Usd,
                "METRIC_FEE4_ADVERTISER" => ParametersMetricsItems::MetricFee4Advertiser,
                "METRIC_FEE4_PARTNER" => ParametersMetricsItems::MetricFee4Partner,
                "METRIC_FEE4_USD" => ParametersMetricsItems::MetricFee4Usd,
                "METRIC_FEE5_ADVERTISER" => ParametersMetricsItems::MetricFee5Advertiser,
                "METRIC_FEE5_PARTNER" => ParametersMetricsItems::MetricFee5Partner,
                "METRIC_FEE5_USD" => ParametersMetricsItems::MetricFee5Usd,
                "METRIC_FEE6_ADVERTISER" => ParametersMetricsItems::MetricFee6Advertiser,
                "METRIC_FEE6_PARTNER" => ParametersMetricsItems::MetricFee6Partner,
                "METRIC_FEE6_USD" => ParametersMetricsItems::MetricFee6Usd,
                "METRIC_FEE7_ADVERTISER" => ParametersMetricsItems::MetricFee7Advertiser,
                "METRIC_FEE7_PARTNER" => ParametersMetricsItems::MetricFee7Partner,
                "METRIC_FEE7_USD" => ParametersMetricsItems::MetricFee7Usd,
                "METRIC_FEE8_ADVERTISER" => ParametersMetricsItems::MetricFee8Advertiser,
                "METRIC_FEE8_PARTNER" => ParametersMetricsItems::MetricFee8Partner,
                "METRIC_FEE8_USD" => ParametersMetricsItems::MetricFee8Usd,
                "METRIC_FEE9_ADVERTISER" => ParametersMetricsItems::MetricFee9Advertiser,
                "METRIC_FEE9_PARTNER" => ParametersMetricsItems::MetricFee9Partner,
                "METRIC_FEE9_USD" => ParametersMetricsItems::MetricFee9Usd,
                "METRIC_FLOODLIGHT_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricFloodlightImpressions
                }
                "METRIC_IMPRESSIONS" => ParametersMetricsItems::MetricImpressions,
                "METRIC_IMPRESSIONS_TO_CONVERSION_RATE" => {
                    ParametersMetricsItems::MetricImpressionsToConversionRate
                }
                "METRIC_LAST_CLICKS" => ParametersMetricsItems::MetricLastClicks,
                "METRIC_LAST_IMPRESSIONS" => ParametersMetricsItems::MetricLastImpressions,
                "METRIC_MEDIA_COST_ADVERTISER" => ParametersMetricsItems::MetricMediaCostAdvertiser,
                "METRIC_MEDIA_COST_ECPAPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostEcpapcAdvertiser
                }
                "METRIC_MEDIA_COST_ECPAPC_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostEcpapcPartner
                }
                "METRIC_MEDIA_COST_ECPAPC_USD" => ParametersMetricsItems::MetricMediaCostEcpapcUsd,
                "METRIC_MEDIA_COST_ECPAPV_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostEcpapvAdvertiser
                }
                "METRIC_MEDIA_COST_ECPAPV_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostEcpapvPartner
                }
                "METRIC_MEDIA_COST_ECPAPV_USD" => ParametersMetricsItems::MetricMediaCostEcpapvUsd,
                "METRIC_MEDIA_COST_ECPA_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostEcpaAdvertiser
                }
                "METRIC_MEDIA_COST_ECPA_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostEcpaPartner
                }
                "METRIC_MEDIA_COST_ECPA_USD" => ParametersMetricsItems::MetricMediaCostEcpaUsd,
                "METRIC_MEDIA_COST_ECPCV_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostEcpcvAdvertiser
                }
                "METRIC_MEDIA_COST_ECPCV_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostEcpcvPartner
                }
                "METRIC_MEDIA_COST_ECPCV_USD" => ParametersMetricsItems::MetricMediaCostEcpcvUsd,
                "METRIC_MEDIA_COST_ECPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostEcpcAdvertiser
                }
                "METRIC_MEDIA_COST_ECPC_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostEcpcPartner
                }
                "METRIC_MEDIA_COST_ECPC_USD" => ParametersMetricsItems::MetricMediaCostEcpcUsd,
                "METRIC_MEDIA_COST_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostEcpmAdvertiser
                }
                "METRIC_MEDIA_COST_ECPM_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostEcpmPartner
                }
                "METRIC_MEDIA_COST_ECPM_USD" => ParametersMetricsItems::MetricMediaCostEcpmUsd,
                "METRIC_MEDIA_COST_PARTNER" => ParametersMetricsItems::MetricMediaCostPartner,
                "METRIC_MEDIA_COST_USD" => ParametersMetricsItems::MetricMediaCostUsd,
                "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricMediaCostViewableEcpmAdvertiser
                }
                "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => {
                    ParametersMetricsItems::MetricMediaCostViewableEcpmPartner
                }
                "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD" => {
                    ParametersMetricsItems::MetricMediaCostViewableEcpmUsd
                }
                "METRIC_MEDIA_FEE1_ADVERTISER" => ParametersMetricsItems::MetricMediaFee1Advertiser,
                "METRIC_MEDIA_FEE1_PARTNER" => ParametersMetricsItems::MetricMediaFee1Partner,
                "METRIC_MEDIA_FEE1_USD" => ParametersMetricsItems::MetricMediaFee1Usd,
                "METRIC_MEDIA_FEE2_ADVERTISER" => ParametersMetricsItems::MetricMediaFee2Advertiser,
                "METRIC_MEDIA_FEE2_PARTNER" => ParametersMetricsItems::MetricMediaFee2Partner,
                "METRIC_MEDIA_FEE2_USD" => ParametersMetricsItems::MetricMediaFee2Usd,
                "METRIC_MEDIA_FEE3_ADVERTISER" => ParametersMetricsItems::MetricMediaFee3Advertiser,
                "METRIC_MEDIA_FEE3_PARTNER" => ParametersMetricsItems::MetricMediaFee3Partner,
                "METRIC_MEDIA_FEE3_USD" => ParametersMetricsItems::MetricMediaFee3Usd,
                "METRIC_MEDIA_FEE4_ADVERTISER" => ParametersMetricsItems::MetricMediaFee4Advertiser,
                "METRIC_MEDIA_FEE4_PARTNER" => ParametersMetricsItems::MetricMediaFee4Partner,
                "METRIC_MEDIA_FEE4_USD" => ParametersMetricsItems::MetricMediaFee4Usd,
                "METRIC_MEDIA_FEE5_ADVERTISER" => ParametersMetricsItems::MetricMediaFee5Advertiser,
                "METRIC_MEDIA_FEE5_PARTNER" => ParametersMetricsItems::MetricMediaFee5Partner,
                "METRIC_MEDIA_FEE5_USD" => ParametersMetricsItems::MetricMediaFee5Usd,
                "METRIC_PIXEL_LOADS" => ParametersMetricsItems::MetricPixelLoads,
                "METRIC_PLATFORM_FEE_ADVERTISER" => {
                    ParametersMetricsItems::MetricPlatformFeeAdvertiser
                }
                "METRIC_PLATFORM_FEE_PARTNER" => ParametersMetricsItems::MetricPlatformFeePartner,
                "METRIC_PLATFORM_FEE_USD" => ParametersMetricsItems::MetricPlatformFeeUsd,
                "METRIC_POST_CLICK_DFA_REVENUE" => {
                    ParametersMetricsItems::MetricPostClickDfaRevenue
                }
                "METRIC_POST_VIEW_DFA_REVENUE" => ParametersMetricsItems::MetricPostViewDfaRevenue,
                "METRIC_PROFIT_ADVERTISER" => ParametersMetricsItems::MetricProfitAdvertiser,
                "METRIC_PROFIT_ECPAPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricProfitEcpapcAdvertiser
                }
                "METRIC_PROFIT_ECPAPC_PARTNER" => ParametersMetricsItems::MetricProfitEcpapcPartner,
                "METRIC_PROFIT_ECPAPC_USD" => ParametersMetricsItems::MetricProfitEcpapcUsd,
                "METRIC_PROFIT_ECPAPV_ADVERTISER" => {
                    ParametersMetricsItems::MetricProfitEcpapvAdvertiser
                }
                "METRIC_PROFIT_ECPAPV_PARTNER" => ParametersMetricsItems::MetricProfitEcpapvPartner,
                "METRIC_PROFIT_ECPAPV_USD" => ParametersMetricsItems::MetricProfitEcpapvUsd,
                "METRIC_PROFIT_ECPA_ADVERTISER" => {
                    ParametersMetricsItems::MetricProfitEcpaAdvertiser
                }
                "METRIC_PROFIT_ECPA_PARTNER" => ParametersMetricsItems::MetricProfitEcpaPartner,
                "METRIC_PROFIT_ECPA_USD" => ParametersMetricsItems::MetricProfitEcpaUsd,
                "METRIC_PROFIT_ECPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricProfitEcpcAdvertiser
                }
                "METRIC_PROFIT_ECPC_PARTNER" => ParametersMetricsItems::MetricProfitEcpcPartner,
                "METRIC_PROFIT_ECPC_USD" => ParametersMetricsItems::MetricProfitEcpcUsd,
                "METRIC_PROFIT_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricProfitEcpmAdvertiser
                }
                "METRIC_PROFIT_ECPM_PARTNER" => ParametersMetricsItems::MetricProfitEcpmPartner,
                "METRIC_PROFIT_ECPM_USD" => ParametersMetricsItems::MetricProfitEcpmUsd,
                "METRIC_PROFIT_MARGIN" => ParametersMetricsItems::MetricProfitMargin,
                "METRIC_PROFIT_PARTNER" => ParametersMetricsItems::MetricProfitPartner,
                "METRIC_PROFIT_USD" => ParametersMetricsItems::MetricProfitUsd,
                "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricProfitViewableEcpmAdvertiser
                }
                "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER" => {
                    ParametersMetricsItems::MetricProfitViewableEcpmPartner
                }
                "METRIC_PROFIT_VIEWABLE_ECPM_USD" => {
                    ParametersMetricsItems::MetricProfitViewableEcpmUsd
                }
                "METRIC_REACH_COOKIE_FREQUENCY" => {
                    ParametersMetricsItems::MetricReachCookieFrequency
                }
                "METRIC_REACH_COOKIE_REACH" => ParametersMetricsItems::MetricReachCookieReach,
                "METRIC_REVENUE_ADVERTISER" => ParametersMetricsItems::MetricRevenueAdvertiser,
                "METRIC_REVENUE_ECPAPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpapcAdvertiser
                }
                "METRIC_REVENUE_ECPAPC_PARTNER" => {
                    ParametersMetricsItems::MetricRevenueEcpapcPartner
                }
                "METRIC_REVENUE_ECPAPC_USD" => ParametersMetricsItems::MetricRevenueEcpapcUsd,
                "METRIC_REVENUE_ECPAPV_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpapvAdvertiser
                }
                "METRIC_REVENUE_ECPAPV_PARTNER" => {
                    ParametersMetricsItems::MetricRevenueEcpapvPartner
                }
                "METRIC_REVENUE_ECPAPV_USD" => ParametersMetricsItems::MetricRevenueEcpapvUsd,
                "METRIC_REVENUE_ECPA_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpaAdvertiser
                }
                "METRIC_REVENUE_ECPA_PARTNER" => ParametersMetricsItems::MetricRevenueEcpaPartner,
                "METRIC_REVENUE_ECPA_USD" => ParametersMetricsItems::MetricRevenueEcpaUsd,
                "METRIC_REVENUE_ECPCV_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpcvAdvertiser
                }
                "METRIC_REVENUE_ECPCV_PARTNER" => ParametersMetricsItems::MetricRevenueEcpcvPartner,
                "METRIC_REVENUE_ECPCV_USD" => ParametersMetricsItems::MetricRevenueEcpcvUsd,
                "METRIC_REVENUE_ECPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpcAdvertiser
                }
                "METRIC_REVENUE_ECPC_PARTNER" => ParametersMetricsItems::MetricRevenueEcpcPartner,
                "METRIC_REVENUE_ECPC_USD" => ParametersMetricsItems::MetricRevenueEcpcUsd,
                "METRIC_REVENUE_ECPIAVC_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpiavcAdvertiser
                }
                "METRIC_REVENUE_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueEcpmAdvertiser
                }
                "METRIC_REVENUE_ECPM_PARTNER" => ParametersMetricsItems::MetricRevenueEcpmPartner,
                "METRIC_REVENUE_ECPM_USD" => ParametersMetricsItems::MetricRevenueEcpmUsd,
                "METRIC_REVENUE_PARTNER" => ParametersMetricsItems::MetricRevenuePartner,
                "METRIC_REVENUE_USD" => ParametersMetricsItems::MetricRevenueUsd,
                "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricRevenueViewableEcpmAdvertiser
                }
                "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER" => {
                    ParametersMetricsItems::MetricRevenueViewableEcpmPartner
                }
                "METRIC_REVENUE_VIEWABLE_ECPM_USD" => {
                    ParametersMetricsItems::MetricRevenueViewableEcpmUsd
                }
                "METRIC_RICH_MEDIA_SCROLLS" => ParametersMetricsItems::MetricRichMediaScrolls,
                "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS" => {
                    ParametersMetricsItems::MetricRichMediaVideoCompletions
                }
                "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES" => {
                    ParametersMetricsItems::MetricRichMediaVideoFirstQuartileCompletes
                }
                "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS" => {
                    ParametersMetricsItems::MetricRichMediaVideoFullScreens
                }
                "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS" => {
                    ParametersMetricsItems::MetricRichMediaVideoMidpoints
                }
                "METRIC_RICH_MEDIA_VIDEO_MUTES" => {
                    ParametersMetricsItems::MetricRichMediaVideoMutes
                }
                "METRIC_RICH_MEDIA_VIDEO_PAUSES" => {
                    ParametersMetricsItems::MetricRichMediaVideoPauses
                }
                "METRIC_RICH_MEDIA_VIDEO_PLAYS" => {
                    ParametersMetricsItems::MetricRichMediaVideoPlays
                }
                "METRIC_RICH_MEDIA_VIDEO_SKIPS" => {
                    ParametersMetricsItems::MetricRichMediaVideoSkips
                }
                "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES" => {
                    ParametersMetricsItems::MetricRichMediaVideoThirdQuartileCompletes
                }
                "METRIC_TEA_TRUEVIEW_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricTeaTrueviewImpressions
                }
                "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES" => {
                    ParametersMetricsItems::MetricTeaTrueviewUniqueCookies
                }
                "METRIC_TOTAL_CONVERSIONS" => ParametersMetricsItems::MetricTotalConversions,
                "METRIC_TOTAL_MEDIA_COST_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpapcAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpapcPartner
                }
                "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpapcUsd
                }
                "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpapvAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpapvPartner
                }
                "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpapvUsd
                }
                "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpaAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpaPartner
                }
                "METRIC_TOTAL_MEDIA_COST_ECPA_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpaUsd
                }
                "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpcvAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpcvPartner
                }
                "METRIC_TOTAL_MEDIA_COST_ECPCV_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpcvUsd
                }
                "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpcAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpcPartner
                }
                "METRIC_TOTAL_MEDIA_COST_ECPC_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpcUsd
                }
                "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpmAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpmPartner
                }
                "METRIC_TOTAL_MEDIA_COST_ECPM_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostEcpmUsd
                }
                "METRIC_TOTAL_MEDIA_COST_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostPartner
                }
                "METRIC_TOTAL_MEDIA_COST_USD" => ParametersMetricsItems::MetricTotalMediaCostUsd,
                "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => {
                    ParametersMetricsItems::MetricTotalMediaCostViewableEcpmAdvertiser
                }
                "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => {
                    ParametersMetricsItems::MetricTotalMediaCostViewableEcpmPartner
                }
                "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD" => {
                    ParametersMetricsItems::MetricTotalMediaCostViewableEcpmUsd
                }
                "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewAverageCpeAdvertiser
                }
                "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER" => {
                    ParametersMetricsItems::MetricTrueviewAverageCpePartner
                }
                "METRIC_TRUEVIEW_AVERAGE_CPE_USD" => {
                    ParametersMetricsItems::MetricTrueviewAverageCpeUsd
                }
                "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewConversionCostManyPerViewAdvertiser
                }
                "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER" => {
                    ParametersMetricsItems::MetricTrueviewConversionCostManyPerViewPartner
                }
                "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD" => {
                    ParametersMetricsItems::MetricTrueviewConversionCostManyPerViewUsd
                }
                "METRIC_TRUEVIEW_CONVERSION_COST_ONE_PER_VIEW_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewConversionCostOnePerViewAdvertiser
                }
                "METRIC_TRUEVIEW_CONVERSION_COST_ONE_PER_VIEW_PARTNER" => {
                    ParametersMetricsItems::MetricTrueviewConversionCostOnePerViewPartner
                }
                "METRIC_TRUEVIEW_CONVERSION_COST_ONE_PER_VIEW_USD" => {
                    ParametersMetricsItems::MetricTrueviewConversionCostOnePerViewUsd
                }
                "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW" => {
                    ParametersMetricsItems::MetricTrueviewConversionManyPerView
                }
                "METRIC_TRUEVIEW_CONVERSION_ONE_PER_VIEW" => {
                    ParametersMetricsItems::MetricTrueviewConversionOnePerView
                }
                "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW" => {
                    ParametersMetricsItems::MetricTrueviewConversionRateOnePerView
                }
                "METRIC_TRUEVIEW_CONVERSION_VALUE_MANY_PER_VIEW_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewConversionValueManyPerViewAdvertiser
                }
                "METRIC_TRUEVIEW_CONVERSION_VALUE_MANY_PER_VIEW_PARTNER" => {
                    ParametersMetricsItems::MetricTrueviewConversionValueManyPerViewPartner
                }
                "METRIC_TRUEVIEW_CONVERSION_VALUE_MANY_PER_VIEW_USD" => {
                    ParametersMetricsItems::MetricTrueviewConversionValueManyPerViewUsd
                }
                "METRIC_TRUEVIEW_CONVERSION_VALUE_ONE_PER_VIEW_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewConversionValueOnePerViewAdvertiser
                }
                "METRIC_TRUEVIEW_CONVERSION_VALUE_ONE_PER_VIEW_PARTNER" => {
                    ParametersMetricsItems::MetricTrueviewConversionValueOnePerViewPartner
                }
                "METRIC_TRUEVIEW_CONVERSION_VALUE_ONE_PER_VIEW_USD" => {
                    ParametersMetricsItems::MetricTrueviewConversionValueOnePerViewUsd
                }
                "METRIC_TRUEVIEW_COST_CONVERSION_MANY_PER_VIEW_RATIO" => {
                    ParametersMetricsItems::MetricTrueviewCostConversionManyPerViewRatio
                }
                "METRIC_TRUEVIEW_COST_CONVERSION_ONE_PER_VIEW_RATIO" => {
                    ParametersMetricsItems::MetricTrueviewCostConversionOnePerViewRatio
                }
                "METRIC_TRUEVIEW_CPV_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewCpvAdvertiser
                }
                "METRIC_TRUEVIEW_CPV_PARTNER" => ParametersMetricsItems::MetricTrueviewCpvPartner,
                "METRIC_TRUEVIEW_CPV_USD" => ParametersMetricsItems::MetricTrueviewCpvUsd,
                "METRIC_TRUEVIEW_EARNED_LIKES" => ParametersMetricsItems::MetricTrueviewEarnedLikes,
                "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS" => {
                    ParametersMetricsItems::MetricTrueviewEarnedPlaylistAdditions
                }
                "METRIC_TRUEVIEW_EARNED_SHARES" => {
                    ParametersMetricsItems::MetricTrueviewEarnedShares
                }
                "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS" => {
                    ParametersMetricsItems::MetricTrueviewEarnedSubscribers
                }
                "METRIC_TRUEVIEW_EARNED_VIEWS" => ParametersMetricsItems::MetricTrueviewEarnedViews,
                "METRIC_TRUEVIEW_ENGAGEMENTS" => ParametersMetricsItems::MetricTrueviewEngagements,
                "METRIC_TRUEVIEW_ENGAGEMENT_RATE" => {
                    ParametersMetricsItems::MetricTrueviewEngagementRate
                }
                "METRIC_TRUEVIEW_IMPRESSION_SHARE" => {
                    ParametersMetricsItems::MetricTrueviewImpressionShare
                }
                "METRIC_TRUEVIEW_LOST_IS_BUDGET" => {
                    ParametersMetricsItems::MetricTrueviewLostIsBudget
                }
                "METRIC_TRUEVIEW_LOST_IS_RANK" => ParametersMetricsItems::MetricTrueviewLostIsRank,
                "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUE" => {
                    ParametersMetricsItems::MetricTrueviewTotalConversionValue
                }
                "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER" => {
                    ParametersMetricsItems::MetricTrueviewTotalConversionValuesAdvertiser
                }
                "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER" => {
                    ParametersMetricsItems::MetricTrueviewTotalConversionValuesPartner
                }
                "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD" => {
                    ParametersMetricsItems::MetricTrueviewTotalConversionValuesUsd
                }
                "METRIC_TRUEVIEW_UNIQUE_VIEWERS" => {
                    ParametersMetricsItems::MetricTrueviewUniqueViewers
                }
                "METRIC_TRUEVIEW_VALUE_CONVERSION_MANY_PER_VIEW_RATIO" => {
                    ParametersMetricsItems::MetricTrueviewValueConversionManyPerViewRatio
                }
                "METRIC_TRUEVIEW_VALUE_CONVERSION_ONE_PER_VIEW_RATIO" => {
                    ParametersMetricsItems::MetricTrueviewValueConversionOnePerViewRatio
                }
                "METRIC_TRUEVIEW_VIEWS" => ParametersMetricsItems::MetricTrueviewViews,
                "METRIC_TRUEVIEW_VIEW_RATE" => ParametersMetricsItems::MetricTrueviewViewRate,
                "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION" => {
                    ParametersMetricsItems::MetricTrueviewViewThroughConversion
                }
                "METRIC_UNIQUE_VISITORS_COOKIES" => {
                    ParametersMetricsItems::MetricUniqueVisitorsCookies
                }
                "METRIC_UNKNOWN" => ParametersMetricsItems::MetricUnknown,
                "METRIC_VIDEO_COMPANION_CLICKS" => {
                    ParametersMetricsItems::MetricVideoCompanionClicks
                }
                "METRIC_VIDEO_COMPANION_IMPRESSIONS" => {
                    ParametersMetricsItems::MetricVideoCompanionImpressions
                }
                "METRIC_VIDEO_COMPLETION_RATE" => ParametersMetricsItems::MetricVideoCompletionRate,
                "METRIC_VIEWABLE_BID_REQUESTS" => ParametersMetricsItems::MetricViewableBidRequests,
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
    pub enum ParametersType {
        TypeActiveGrp,
        TypeAudienceComposition,
        TypeAudiencePerformance,
        TypeClientSafe,
        TypeComscoreVce,
        TypeCrossFee,
        TypeCrossPartner,
        TypeCrossPartnerThirdPartyDataProvider,
        TypeEstimatedConversion,
        TypeFee,
        TypeGeneral,
        TypeInventoryAvailability,
        TypeKeyword,
        TypeLinearTvSearchLift,
        TypeNielsenAudienceProfile,
        TypeNielsenDailyReachBuild,
        TypeNielsenOnlineGlobalMarket,
        TypeNielsenSite,
        TypeNotSupported,
        TypeOrderId,
        TypePageCategory,
        TypePetraNielsenAudienceProfile,
        TypePetraNielsenDailyReachBuild,
        TypePetraNielsenOnlineGlobalMarket,
        TypePixelLoad,
        TypeReachAndFrequency,
        TypeReachAudience,
        TypeThirdPartyDataProvider,
        TypeTrueview,
        TypeTrueviewIar,
        TypeVerification,
        TypeYoutubeVertical,
    }
    impl ParametersType {
        pub fn as_str(self) -> &'static str {
            match self {
                ParametersType::TypeActiveGrp => "TYPE_ACTIVE_GRP",
                ParametersType::TypeAudienceComposition => "TYPE_AUDIENCE_COMPOSITION",
                ParametersType::TypeAudiencePerformance => "TYPE_AUDIENCE_PERFORMANCE",
                ParametersType::TypeClientSafe => "TYPE_CLIENT_SAFE",
                ParametersType::TypeComscoreVce => "TYPE_COMSCORE_VCE",
                ParametersType::TypeCrossFee => "TYPE_CROSS_FEE",
                ParametersType::TypeCrossPartner => "TYPE_CROSS_PARTNER",
                ParametersType::TypeCrossPartnerThirdPartyDataProvider => {
                    "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER"
                }
                ParametersType::TypeEstimatedConversion => "TYPE_ESTIMATED_CONVERSION",
                ParametersType::TypeFee => "TYPE_FEE",
                ParametersType::TypeGeneral => "TYPE_GENERAL",
                ParametersType::TypeInventoryAvailability => "TYPE_INVENTORY_AVAILABILITY",
                ParametersType::TypeKeyword => "TYPE_KEYWORD",
                ParametersType::TypeLinearTvSearchLift => "TYPE_LINEAR_TV_SEARCH_LIFT",
                ParametersType::TypeNielsenAudienceProfile => "TYPE_NIELSEN_AUDIENCE_PROFILE",
                ParametersType::TypeNielsenDailyReachBuild => "TYPE_NIELSEN_DAILY_REACH_BUILD",
                ParametersType::TypeNielsenOnlineGlobalMarket => {
                    "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET"
                }
                ParametersType::TypeNielsenSite => "TYPE_NIELSEN_SITE",
                ParametersType::TypeNotSupported => "TYPE_NOT_SUPPORTED",
                ParametersType::TypeOrderId => "TYPE_ORDER_ID",
                ParametersType::TypePageCategory => "TYPE_PAGE_CATEGORY",
                ParametersType::TypePetraNielsenAudienceProfile => {
                    "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE"
                }
                ParametersType::TypePetraNielsenDailyReachBuild => {
                    "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD"
                }
                ParametersType::TypePetraNielsenOnlineGlobalMarket => {
                    "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET"
                }
                ParametersType::TypePixelLoad => "TYPE_PIXEL_LOAD",
                ParametersType::TypeReachAndFrequency => "TYPE_REACH_AND_FREQUENCY",
                ParametersType::TypeReachAudience => "TYPE_REACH_AUDIENCE",
                ParametersType::TypeThirdPartyDataProvider => "TYPE_THIRD_PARTY_DATA_PROVIDER",
                ParametersType::TypeTrueview => "TYPE_TRUEVIEW",
                ParametersType::TypeTrueviewIar => "TYPE_TRUEVIEW_IAR",
                ParametersType::TypeVerification => "TYPE_VERIFICATION",
                ParametersType::TypeYoutubeVertical => "TYPE_YOUTUBE_VERTICAL",
            }
        }
    }
    impl ::std::fmt::Display for ParametersType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParametersType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParametersType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_ACTIVE_GRP" => ParametersType::TypeActiveGrp,
                "TYPE_AUDIENCE_COMPOSITION" => ParametersType::TypeAudienceComposition,
                "TYPE_AUDIENCE_PERFORMANCE" => ParametersType::TypeAudiencePerformance,
                "TYPE_CLIENT_SAFE" => ParametersType::TypeClientSafe,
                "TYPE_COMSCORE_VCE" => ParametersType::TypeComscoreVce,
                "TYPE_CROSS_FEE" => ParametersType::TypeCrossFee,
                "TYPE_CROSS_PARTNER" => ParametersType::TypeCrossPartner,
                "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER" => {
                    ParametersType::TypeCrossPartnerThirdPartyDataProvider
                }
                "TYPE_ESTIMATED_CONVERSION" => ParametersType::TypeEstimatedConversion,
                "TYPE_FEE" => ParametersType::TypeFee,
                "TYPE_GENERAL" => ParametersType::TypeGeneral,
                "TYPE_INVENTORY_AVAILABILITY" => ParametersType::TypeInventoryAvailability,
                "TYPE_KEYWORD" => ParametersType::TypeKeyword,
                "TYPE_LINEAR_TV_SEARCH_LIFT" => ParametersType::TypeLinearTvSearchLift,
                "TYPE_NIELSEN_AUDIENCE_PROFILE" => ParametersType::TypeNielsenAudienceProfile,
                "TYPE_NIELSEN_DAILY_REACH_BUILD" => ParametersType::TypeNielsenDailyReachBuild,
                "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET" => {
                    ParametersType::TypeNielsenOnlineGlobalMarket
                }
                "TYPE_NIELSEN_SITE" => ParametersType::TypeNielsenSite,
                "TYPE_NOT_SUPPORTED" => ParametersType::TypeNotSupported,
                "TYPE_ORDER_ID" => ParametersType::TypeOrderId,
                "TYPE_PAGE_CATEGORY" => ParametersType::TypePageCategory,
                "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE" => {
                    ParametersType::TypePetraNielsenAudienceProfile
                }
                "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD" => {
                    ParametersType::TypePetraNielsenDailyReachBuild
                }
                "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET" => {
                    ParametersType::TypePetraNielsenOnlineGlobalMarket
                }
                "TYPE_PIXEL_LOAD" => ParametersType::TypePixelLoad,
                "TYPE_REACH_AND_FREQUENCY" => ParametersType::TypeReachAndFrequency,
                "TYPE_REACH_AUDIENCE" => ParametersType::TypeReachAudience,
                "TYPE_THIRD_PARTY_DATA_PROVIDER" => ParametersType::TypeThirdPartyDataProvider,
                "TYPE_TRUEVIEW" => ParametersType::TypeTrueview,
                "TYPE_TRUEVIEW_IAR" => ParametersType::TypeTrueviewIar,
                "TYPE_VERIFICATION" => ParametersType::TypeVerification,
                "TYPE_YOUTUBE_VERTICAL" => ParametersType::TypeYoutubeVertical,
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
    pub struct Parameters {
        #[doc = "Filters used to match traffic data in your report."]
        #[serde(rename = "filters", default)]
        pub filters: Option<Vec<crate::schemas::FilterPair>>,
        #[doc = "Data is grouped by the filters listed in this field."]
        #[serde(rename = "groupBys", default)]
        pub group_bys: Option<Vec<crate::schemas::ParametersGroupBysItems>>,
        #[doc = "Deprecated. This field is no longer in use."]
        #[serde(rename = "includeInviteData", default)]
        pub include_invite_data: Option<bool>,
        #[doc = "Metrics to include as columns in your report."]
        #[serde(rename = "metrics", default)]
        pub metrics: Option<Vec<crate::schemas::ParametersMetricsItems>>,
        #[doc = "Report type."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::ParametersType>,
    }
    impl ::field_selector::FieldSelector for Parameters {
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
    pub struct Query {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#query\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Query metadata."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::QueryMetadata>,
        #[doc = "Query parameters."]
        #[serde(rename = "params", default)]
        pub params: Option<crate::schemas::Parameters>,
        #[doc = "Query ID."]
        #[serde(rename = "queryId", default)]
        #[serde(with = "crate::parsed_string")]
        pub query_id: Option<i64>,
        #[doc = "The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(rename = "reportDataEndTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_data_end_time_ms: Option<i64>,
        #[doc = "The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(rename = "reportDataStartTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_data_start_time_ms: Option<i64>,
        #[doc = "Information on how often and when to run a query."]
        #[serde(rename = "schedule", default)]
        pub schedule: Option<crate::schemas::QuerySchedule>,
        #[doc = "Canonical timezone code for report data time. Defaults to America/New_York."]
        #[serde(rename = "timezoneCode", default)]
        pub timezone_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for Query {
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
    pub enum QueryMetadataDataRange {
        AllTime,
        CurrentDay,
        CustomDates,
        Last14Days,
        Last30Days,
        Last365Days,
        Last7Days,
        Last90Days,
        MonthToDate,
        PreviousDay,
        PreviousHalfMonth,
        PreviousMonth,
        PreviousQuarter,
        PreviousWeek,
        PreviousYear,
        QuarterToDate,
        TypeNotSupported,
        WeekToDate,
        YearToDate,
    }
    impl QueryMetadataDataRange {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryMetadataDataRange::AllTime => "ALL_TIME",
                QueryMetadataDataRange::CurrentDay => "CURRENT_DAY",
                QueryMetadataDataRange::CustomDates => "CUSTOM_DATES",
                QueryMetadataDataRange::Last14Days => "LAST_14_DAYS",
                QueryMetadataDataRange::Last30Days => "LAST_30_DAYS",
                QueryMetadataDataRange::Last365Days => "LAST_365_DAYS",
                QueryMetadataDataRange::Last7Days => "LAST_7_DAYS",
                QueryMetadataDataRange::Last90Days => "LAST_90_DAYS",
                QueryMetadataDataRange::MonthToDate => "MONTH_TO_DATE",
                QueryMetadataDataRange::PreviousDay => "PREVIOUS_DAY",
                QueryMetadataDataRange::PreviousHalfMonth => "PREVIOUS_HALF_MONTH",
                QueryMetadataDataRange::PreviousMonth => "PREVIOUS_MONTH",
                QueryMetadataDataRange::PreviousQuarter => "PREVIOUS_QUARTER",
                QueryMetadataDataRange::PreviousWeek => "PREVIOUS_WEEK",
                QueryMetadataDataRange::PreviousYear => "PREVIOUS_YEAR",
                QueryMetadataDataRange::QuarterToDate => "QUARTER_TO_DATE",
                QueryMetadataDataRange::TypeNotSupported => "TYPE_NOT_SUPPORTED",
                QueryMetadataDataRange::WeekToDate => "WEEK_TO_DATE",
                QueryMetadataDataRange::YearToDate => "YEAR_TO_DATE",
            }
        }
    }
    impl ::std::fmt::Display for QueryMetadataDataRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryMetadataDataRange {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryMetadataDataRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => QueryMetadataDataRange::AllTime,
                "CURRENT_DAY" => QueryMetadataDataRange::CurrentDay,
                "CUSTOM_DATES" => QueryMetadataDataRange::CustomDates,
                "LAST_14_DAYS" => QueryMetadataDataRange::Last14Days,
                "LAST_30_DAYS" => QueryMetadataDataRange::Last30Days,
                "LAST_365_DAYS" => QueryMetadataDataRange::Last365Days,
                "LAST_7_DAYS" => QueryMetadataDataRange::Last7Days,
                "LAST_90_DAYS" => QueryMetadataDataRange::Last90Days,
                "MONTH_TO_DATE" => QueryMetadataDataRange::MonthToDate,
                "PREVIOUS_DAY" => QueryMetadataDataRange::PreviousDay,
                "PREVIOUS_HALF_MONTH" => QueryMetadataDataRange::PreviousHalfMonth,
                "PREVIOUS_MONTH" => QueryMetadataDataRange::PreviousMonth,
                "PREVIOUS_QUARTER" => QueryMetadataDataRange::PreviousQuarter,
                "PREVIOUS_WEEK" => QueryMetadataDataRange::PreviousWeek,
                "PREVIOUS_YEAR" => QueryMetadataDataRange::PreviousYear,
                "QUARTER_TO_DATE" => QueryMetadataDataRange::QuarterToDate,
                "TYPE_NOT_SUPPORTED" => QueryMetadataDataRange::TypeNotSupported,
                "WEEK_TO_DATE" => QueryMetadataDataRange::WeekToDate,
                "YEAR_TO_DATE" => QueryMetadataDataRange::YearToDate,
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
    pub enum QueryMetadataFormat {
        Csv,
        ExcelCsv,
        Xlsx,
    }
    impl QueryMetadataFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryMetadataFormat::Csv => "CSV",
                QueryMetadataFormat::ExcelCsv => "EXCEL_CSV",
                QueryMetadataFormat::Xlsx => "XLSX",
            }
        }
    }
    impl ::std::fmt::Display for QueryMetadataFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryMetadataFormat {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryMetadataFormat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => QueryMetadataFormat::Csv,
                "EXCEL_CSV" => QueryMetadataFormat::ExcelCsv,
                "XLSX" => QueryMetadataFormat::Xlsx,
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
    pub struct QueryMetadata {
        #[doc = "Range of report data."]
        #[serde(rename = "dataRange", default)]
        pub data_range: Option<crate::schemas::QueryMetadataDataRange>,
        #[doc = "Format of the generated report."]
        #[serde(rename = "format", default)]
        pub format: Option<crate::schemas::QueryMetadataFormat>,
        #[doc = "The path to the location in Google Cloud Storage where the latest report is stored."]
        #[serde(rename = "googleCloudStoragePathForLatestReport", default)]
        pub google_cloud_storage_path_for_latest_report: Option<String>,
        #[doc = "The path in Google Drive for the latest report."]
        #[serde(rename = "googleDrivePathForLatestReport", default)]
        pub google_drive_path_for_latest_report: Option<String>,
        #[doc = "The time when the latest report started to run."]
        #[serde(rename = "latestReportRunTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub latest_report_run_time_ms: Option<i64>,
        #[doc = "Locale of the generated reports. Valid values are cs CZECH de GERMAN en ENGLISH es SPANISH fr FRENCH it ITALIAN ja JAPANESE ko KOREAN pl POLISH pt-BR BRAZILIAN_PORTUGUESE ru RUSSIAN tr TURKISH uk UKRAINIAN zh-CN CHINA_CHINESE zh-TW TAIWAN_CHINESE\n\nAn locale string not in the list above will generate reports in English."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[doc = "Number of reports that have been generated for the query."]
        #[serde(rename = "reportCount", default)]
        pub report_count: Option<i32>,
        #[doc = "Whether the latest report is currently running."]
        #[serde(rename = "running", default)]
        pub running: Option<bool>,
        #[doc = "Whether to send an email notification when a report is ready. Default to false."]
        #[serde(rename = "sendNotification", default)]
        pub send_notification: Option<bool>,
        #[doc = "List of email addresses which are sent email notifications when the report is finished. Separate from sendNotification."]
        #[serde(rename = "shareEmailAddress", default)]
        pub share_email_address: Option<Vec<String>>,
        #[doc = "Query title. It is used to name the reports generated from this query."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for QueryMetadata {
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
    pub enum QueryScheduleFrequency {
        Daily,
        Monthly,
        OneTime,
        Quarterly,
        SemiMonthly,
        Weekly,
    }
    impl QueryScheduleFrequency {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryScheduleFrequency::Daily => "DAILY",
                QueryScheduleFrequency::Monthly => "MONTHLY",
                QueryScheduleFrequency::OneTime => "ONE_TIME",
                QueryScheduleFrequency::Quarterly => "QUARTERLY",
                QueryScheduleFrequency::SemiMonthly => "SEMI_MONTHLY",
                QueryScheduleFrequency::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::fmt::Display for QueryScheduleFrequency {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryScheduleFrequency {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryScheduleFrequency {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => QueryScheduleFrequency::Daily,
                "MONTHLY" => QueryScheduleFrequency::Monthly,
                "ONE_TIME" => QueryScheduleFrequency::OneTime,
                "QUARTERLY" => QueryScheduleFrequency::Quarterly,
                "SEMI_MONTHLY" => QueryScheduleFrequency::SemiMonthly,
                "WEEKLY" => QueryScheduleFrequency::Weekly,
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
    pub struct QuerySchedule {
        #[doc = "Datetime to periodically run the query until."]
        #[serde(rename = "endTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_ms: Option<i64>,
        #[doc = "How often the query is run."]
        #[serde(rename = "frequency", default)]
        pub frequency: Option<crate::schemas::QueryScheduleFrequency>,
        #[doc = "Time of day at which a new report will be generated, represented as minutes past midnight. Range is 0 to 1439. Only applies to scheduled reports."]
        #[serde(rename = "nextRunMinuteOfDay", default)]
        pub next_run_minute_of_day: Option<i32>,
        #[doc = "Canonical timezone code for report generation time. Defaults to America/New_York."]
        #[serde(rename = "nextRunTimezoneCode", default)]
        pub next_run_timezone_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for QuerySchedule {
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
    pub struct Report {
        #[doc = "Key used to identify a report."]
        #[serde(rename = "key", default)]
        pub key: Option<crate::schemas::ReportKey>,
        #[doc = "Report metadata."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::ReportMetadata>,
        #[doc = "Report parameters."]
        #[serde(rename = "params", default)]
        pub params: Option<crate::schemas::Parameters>,
    }
    impl ::field_selector::FieldSelector for Report {
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
    pub enum ReportFailureErrorCode {
        AuthenticationError,
        DeprecatedReportingInvalidQuery,
        ReportingBucketNotFound,
        ReportingCreateBucketFailed,
        ReportingDeleteBucketFailed,
        ReportingFatalError,
        ReportingIllegalFilename,
        ReportingImcompatibleMetrics,
        ReportingInvalidQueryMissingPartnerAndAdvertiserFilters,
        ReportingInvalidQueryTitleMissing,
        ReportingInvalidQueryTooManyUnfilteredLargeGroupBys,
        ReportingQueryNotFound,
        ReportingTransientError,
        ReportingUpdateBucketPermissionFailed,
        ReportingWriteBucketObjectFailed,
        ServerError,
        UnauthorizedApiAccess,
        ValidationError,
    }
    impl ReportFailureErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportFailureErrorCode::AuthenticationError => "AUTHENTICATION_ERROR",
                ReportFailureErrorCode::DeprecatedReportingInvalidQuery => {
                    "DEPRECATED_REPORTING_INVALID_QUERY"
                }
                ReportFailureErrorCode::ReportingBucketNotFound => "REPORTING_BUCKET_NOT_FOUND",
                ReportFailureErrorCode::ReportingCreateBucketFailed => {
                    "REPORTING_CREATE_BUCKET_FAILED"
                }
                ReportFailureErrorCode::ReportingDeleteBucketFailed => {
                    "REPORTING_DELETE_BUCKET_FAILED"
                }
                ReportFailureErrorCode::ReportingFatalError => "REPORTING_FATAL_ERROR",
                ReportFailureErrorCode::ReportingIllegalFilename => "REPORTING_ILLEGAL_FILENAME",
                ReportFailureErrorCode::ReportingImcompatibleMetrics => {
                    "REPORTING_IMCOMPATIBLE_METRICS"
                }
                ReportFailureErrorCode::ReportingInvalidQueryMissingPartnerAndAdvertiserFilters => {
                    "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS"
                }
                ReportFailureErrorCode::ReportingInvalidQueryTitleMissing => {
                    "REPORTING_INVALID_QUERY_TITLE_MISSING"
                }
                ReportFailureErrorCode::ReportingInvalidQueryTooManyUnfilteredLargeGroupBys => {
                    "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS"
                }
                ReportFailureErrorCode::ReportingQueryNotFound => "REPORTING_QUERY_NOT_FOUND",
                ReportFailureErrorCode::ReportingTransientError => "REPORTING_TRANSIENT_ERROR",
                ReportFailureErrorCode::ReportingUpdateBucketPermissionFailed => {
                    "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED"
                }
                ReportFailureErrorCode::ReportingWriteBucketObjectFailed => {
                    "REPORTING_WRITE_BUCKET_OBJECT_FAILED"
                }
                ReportFailureErrorCode::ServerError => "SERVER_ERROR",
                ReportFailureErrorCode::UnauthorizedApiAccess => "UNAUTHORIZED_API_ACCESS",
                ReportFailureErrorCode::ValidationError => "VALIDATION_ERROR",
            }
        }
    }
    impl ::std::fmt::Display for ReportFailureErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportFailureErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportFailureErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_ERROR" => ReportFailureErrorCode::AuthenticationError,
                "DEPRECATED_REPORTING_INVALID_QUERY" => {
                    ReportFailureErrorCode::DeprecatedReportingInvalidQuery
                }
                "REPORTING_BUCKET_NOT_FOUND" => ReportFailureErrorCode::ReportingBucketNotFound,
                "REPORTING_CREATE_BUCKET_FAILED" => {
                    ReportFailureErrorCode::ReportingCreateBucketFailed
                }
                "REPORTING_DELETE_BUCKET_FAILED" => {
                    ReportFailureErrorCode::ReportingDeleteBucketFailed
                }
                "REPORTING_FATAL_ERROR" => ReportFailureErrorCode::ReportingFatalError,
                "REPORTING_ILLEGAL_FILENAME" => ReportFailureErrorCode::ReportingIllegalFilename,
                "REPORTING_IMCOMPATIBLE_METRICS" => {
                    ReportFailureErrorCode::ReportingImcompatibleMetrics
                }
                "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS" => {
                    ReportFailureErrorCode::ReportingInvalidQueryMissingPartnerAndAdvertiserFilters
                }
                "REPORTING_INVALID_QUERY_TITLE_MISSING" => {
                    ReportFailureErrorCode::ReportingInvalidQueryTitleMissing
                }
                "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS" => {
                    ReportFailureErrorCode::ReportingInvalidQueryTooManyUnfilteredLargeGroupBys
                }
                "REPORTING_QUERY_NOT_FOUND" => ReportFailureErrorCode::ReportingQueryNotFound,
                "REPORTING_TRANSIENT_ERROR" => ReportFailureErrorCode::ReportingTransientError,
                "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED" => {
                    ReportFailureErrorCode::ReportingUpdateBucketPermissionFailed
                }
                "REPORTING_WRITE_BUCKET_OBJECT_FAILED" => {
                    ReportFailureErrorCode::ReportingWriteBucketObjectFailed
                }
                "SERVER_ERROR" => ReportFailureErrorCode::ServerError,
                "UNAUTHORIZED_API_ACCESS" => ReportFailureErrorCode::UnauthorizedApiAccess,
                "VALIDATION_ERROR" => ReportFailureErrorCode::ValidationError,
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
    pub struct ReportFailure {
        #[doc = "Error code that shows why the report was not created."]
        #[serde(rename = "errorCode", default)]
        pub error_code: Option<crate::schemas::ReportFailureErrorCode>,
    }
    impl ::field_selector::FieldSelector for ReportFailure {
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
    pub struct ReportKey {
        #[doc = "Query ID."]
        #[serde(rename = "queryId", default)]
        #[serde(with = "crate::parsed_string")]
        pub query_id: Option<i64>,
        #[doc = "Report ID."]
        #[serde(rename = "reportId", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector for ReportKey {
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
    pub struct ReportMetadata {
        #[doc = "The path to the location in Google Cloud Storage where the report is stored."]
        #[serde(rename = "googleCloudStoragePath", default)]
        pub google_cloud_storage_path: Option<String>,
        #[doc = "The ending time for the data that is shown in the report."]
        #[serde(rename = "reportDataEndTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_data_end_time_ms: Option<i64>,
        #[doc = "The starting time for the data that is shown in the report."]
        #[serde(rename = "reportDataStartTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_data_start_time_ms: Option<i64>,
        #[doc = "Report status."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::ReportStatus>,
    }
    impl ::field_selector::FieldSelector for ReportMetadata {
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
    pub enum ReportStatusFormat {
        Csv,
        ExcelCsv,
        Xlsx,
    }
    impl ReportStatusFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportStatusFormat::Csv => "CSV",
                ReportStatusFormat::ExcelCsv => "EXCEL_CSV",
                ReportStatusFormat::Xlsx => "XLSX",
            }
        }
    }
    impl ::std::fmt::Display for ReportStatusFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportStatusFormat {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportStatusFormat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => ReportStatusFormat::Csv,
                "EXCEL_CSV" => ReportStatusFormat::ExcelCsv,
                "XLSX" => ReportStatusFormat::Xlsx,
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
    pub enum ReportStatusState {
        Done,
        Failed,
        Running,
    }
    impl ReportStatusState {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportStatusState::Done => "DONE",
                ReportStatusState::Failed => "FAILED",
                ReportStatusState::Running => "RUNNING",
            }
        }
    }
    impl ::std::fmt::Display for ReportStatusState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportStatusState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportStatusState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DONE" => ReportStatusState::Done,
                "FAILED" => ReportStatusState::Failed,
                "RUNNING" => ReportStatusState::Running,
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
    pub struct ReportStatus {
        #[doc = "If the report failed, this records the cause."]
        #[serde(rename = "failure", default)]
        pub failure: Option<crate::schemas::ReportFailure>,
        #[doc = "The time when this report either completed successfully or failed."]
        #[serde(rename = "finishTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub finish_time_ms: Option<i64>,
        #[doc = "The file type of the report."]
        #[serde(rename = "format", default)]
        pub format: Option<crate::schemas::ReportStatusFormat>,
        #[doc = "The state of the report."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::ReportStatusState>,
    }
    impl ::field_selector::FieldSelector for ReportStatus {
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
    pub struct RowStatus {
        #[doc = "Whether the stored entity is changed as a result of upload."]
        #[serde(rename = "changed", default)]
        pub changed: Option<bool>,
        #[doc = "Entity Id."]
        #[serde(rename = "entityId", default)]
        #[serde(with = "crate::parsed_string")]
        pub entity_id: Option<i64>,
        #[doc = "Entity name."]
        #[serde(rename = "entityName", default)]
        pub entity_name: Option<String>,
        #[doc = "Reasons why the entity can't be uploaded."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<String>>,
        #[doc = "Whether the entity is persisted."]
        #[serde(rename = "persisted", default)]
        pub persisted: Option<bool>,
        #[doc = "Row number."]
        #[serde(rename = "rowNumber", default)]
        pub row_number: Option<i32>,
    }
    impl ::field_selector::FieldSelector for RowStatus {
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
    pub enum RunQueryRequestDataRange {
        AllTime,
        CurrentDay,
        CustomDates,
        Last14Days,
        Last30Days,
        Last365Days,
        Last7Days,
        Last90Days,
        MonthToDate,
        PreviousDay,
        PreviousHalfMonth,
        PreviousMonth,
        PreviousQuarter,
        PreviousWeek,
        PreviousYear,
        QuarterToDate,
        TypeNotSupported,
        WeekToDate,
        YearToDate,
    }
    impl RunQueryRequestDataRange {
        pub fn as_str(self) -> &'static str {
            match self {
                RunQueryRequestDataRange::AllTime => "ALL_TIME",
                RunQueryRequestDataRange::CurrentDay => "CURRENT_DAY",
                RunQueryRequestDataRange::CustomDates => "CUSTOM_DATES",
                RunQueryRequestDataRange::Last14Days => "LAST_14_DAYS",
                RunQueryRequestDataRange::Last30Days => "LAST_30_DAYS",
                RunQueryRequestDataRange::Last365Days => "LAST_365_DAYS",
                RunQueryRequestDataRange::Last7Days => "LAST_7_DAYS",
                RunQueryRequestDataRange::Last90Days => "LAST_90_DAYS",
                RunQueryRequestDataRange::MonthToDate => "MONTH_TO_DATE",
                RunQueryRequestDataRange::PreviousDay => "PREVIOUS_DAY",
                RunQueryRequestDataRange::PreviousHalfMonth => "PREVIOUS_HALF_MONTH",
                RunQueryRequestDataRange::PreviousMonth => "PREVIOUS_MONTH",
                RunQueryRequestDataRange::PreviousQuarter => "PREVIOUS_QUARTER",
                RunQueryRequestDataRange::PreviousWeek => "PREVIOUS_WEEK",
                RunQueryRequestDataRange::PreviousYear => "PREVIOUS_YEAR",
                RunQueryRequestDataRange::QuarterToDate => "QUARTER_TO_DATE",
                RunQueryRequestDataRange::TypeNotSupported => "TYPE_NOT_SUPPORTED",
                RunQueryRequestDataRange::WeekToDate => "WEEK_TO_DATE",
                RunQueryRequestDataRange::YearToDate => "YEAR_TO_DATE",
            }
        }
    }
    impl ::std::fmt::Display for RunQueryRequestDataRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RunQueryRequestDataRange {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RunQueryRequestDataRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => RunQueryRequestDataRange::AllTime,
                "CURRENT_DAY" => RunQueryRequestDataRange::CurrentDay,
                "CUSTOM_DATES" => RunQueryRequestDataRange::CustomDates,
                "LAST_14_DAYS" => RunQueryRequestDataRange::Last14Days,
                "LAST_30_DAYS" => RunQueryRequestDataRange::Last30Days,
                "LAST_365_DAYS" => RunQueryRequestDataRange::Last365Days,
                "LAST_7_DAYS" => RunQueryRequestDataRange::Last7Days,
                "LAST_90_DAYS" => RunQueryRequestDataRange::Last90Days,
                "MONTH_TO_DATE" => RunQueryRequestDataRange::MonthToDate,
                "PREVIOUS_DAY" => RunQueryRequestDataRange::PreviousDay,
                "PREVIOUS_HALF_MONTH" => RunQueryRequestDataRange::PreviousHalfMonth,
                "PREVIOUS_MONTH" => RunQueryRequestDataRange::PreviousMonth,
                "PREVIOUS_QUARTER" => RunQueryRequestDataRange::PreviousQuarter,
                "PREVIOUS_WEEK" => RunQueryRequestDataRange::PreviousWeek,
                "PREVIOUS_YEAR" => RunQueryRequestDataRange::PreviousYear,
                "QUARTER_TO_DATE" => RunQueryRequestDataRange::QuarterToDate,
                "TYPE_NOT_SUPPORTED" => RunQueryRequestDataRange::TypeNotSupported,
                "WEEK_TO_DATE" => RunQueryRequestDataRange::WeekToDate,
                "YEAR_TO_DATE" => RunQueryRequestDataRange::YearToDate,
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
    pub struct RunQueryRequest {
        #[doc = "Report data range used to generate the report."]
        #[serde(rename = "dataRange", default)]
        pub data_range: Option<crate::schemas::RunQueryRequestDataRange>,
        #[doc = "The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(rename = "reportDataEndTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_data_end_time_ms: Option<i64>,
        #[doc = "The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(rename = "reportDataStartTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_data_start_time_ms: Option<i64>,
        #[doc = "Canonical timezone code for report data time. Defaults to America/New_York."]
        #[serde(rename = "timezoneCode", default)]
        pub timezone_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for RunQueryRequest {
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
    pub enum UploadLineItemsRequestFormat {
        Csv,
    }
    impl UploadLineItemsRequestFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                UploadLineItemsRequestFormat::Csv => "CSV",
            }
        }
    }
    impl ::std::fmt::Display for UploadLineItemsRequestFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UploadLineItemsRequestFormat {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UploadLineItemsRequestFormat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => UploadLineItemsRequestFormat::Csv,
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
    pub struct UploadLineItemsRequest {
        #[doc = "Set to true to get upload status without actually persisting the line items."]
        #[serde(rename = "dryRun", default)]
        pub dry_run: Option<bool>,
        #[doc = "Format the line items are in. Default to CSV."]
        #[serde(rename = "format", default)]
        pub format: Option<crate::schemas::UploadLineItemsRequestFormat>,
        #[doc = "Line items in CSV to upload. Refer to  Entity Write File Format for more information on file format."]
        #[serde(rename = "lineItems", default)]
        pub line_items: Option<String>,
    }
    impl ::field_selector::FieldSelector for UploadLineItemsRequest {
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
    pub struct UploadLineItemsResponse {
        #[doc = "Status of upload."]
        #[serde(rename = "uploadStatus", default)]
        pub upload_status: Option<crate::schemas::UploadStatus>,
    }
    impl ::field_selector::FieldSelector for UploadLineItemsResponse {
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
    pub struct UploadStatus {
        #[doc = "Reasons why upload can't be completed."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<String>>,
        #[doc = "Per-row upload status."]
        #[serde(rename = "rowStatus", default)]
        pub row_status: Option<Vec<crate::schemas::RowStatus>>,
    }
    impl ::field_selector::FieldSelector for UploadStatus {
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
    #[doc = "Actions that can be performed on the lineitems resource"]
    pub fn lineitems(&self) -> crate::lineitems::LineitemsActions<A> {
        crate::lineitems::LineitemsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the queries resource"]
    pub fn queries(&self) -> crate::queries::QueriesActions<A> {
        crate::queries::QueriesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the reports resource"]
    pub fn reports(&self) -> crate::reports::ReportsActions<A> {
        crate::reports::ReportsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the sdf resource"]
    pub fn sdf(&self) -> crate::sdf::SdfActions<A> {
        crate::sdf::SdfActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod lineitems {
    pub mod params {}
    pub struct LineitemsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> LineitemsActions<'a, A> {
        #[doc = "Retrieves line items in CSV format. TrueView line items are not supported."]
        pub fn downloadlineitems(
            &self,
            request: crate::schemas::DownloadLineItemsRequest,
        ) -> DownloadlineitemsRequestBuilder<A> {
            DownloadlineitemsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Uploads line items in CSV format. TrueView line items are not supported."]
        pub fn uploadlineitems(
            &self,
            request: crate::schemas::UploadLineItemsRequest,
        ) -> UploadlineitemsRequestBuilder<A> {
            UploadlineitemsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DownloadlineitemsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::DownloadLineItemsRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DownloadlineitemsRequestBuilder<'a, A> {
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DownloadLineItemsResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("lineitems/downloadlineitems");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UploadlineitemsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::UploadLineItemsRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UploadlineitemsRequestBuilder<'a, A> {
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::UploadLineItemsResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("lineitems/uploadlineitems");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod queries {
    pub mod params {}
    pub struct QueriesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> QueriesActions<'a, A> {
        #[doc = "Creates a query."]
        pub fn createquery(&self, request: crate::schemas::Query) -> CreatequeryRequestBuilder<A> {
            CreatequeryRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Deletes a stored query as well as the associated stored reports."]
        pub fn deletequery(&self, query_id: i64) -> DeletequeryRequestBuilder<A> {
            DeletequeryRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                query_id,
            }
        }
        #[doc = "Retrieves a stored query."]
        pub fn getquery(&self, query_id: i64) -> GetqueryRequestBuilder<A> {
            GetqueryRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                query_id,
            }
        }
        #[doc = "Retrieves stored queries."]
        pub fn listqueries(&self) -> ListqueriesRequestBuilder<A> {
            ListqueriesRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
        #[doc = "Runs a stored query to generate a report."]
        pub fn runquery(
            &self,
            request: crate::schemas::RunQueryRequest,
            query_id: i64,
        ) -> RunqueryRequestBuilder<A> {
            RunqueryRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                query_id,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreatequeryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Query,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreatequeryRequestBuilder<'a, A> {
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Query, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("query");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeletequeryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        query_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeletequeryRequestBuilder<'a, A> {
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("query/");
            {
                let str_value = self.query_id.to_string();
                output.push_str(&str_value);
            }
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetqueryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        query_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetqueryRequestBuilder<'a, A> {
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
        pub fn execute_debug(self) -> Result<crate::schemas::Query, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("query/");
            {
                let str_value = self.query_id.to_string();
                output.push_str(&str_value);
            }
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListqueriesRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListqueriesRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::ListQueriesResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("queries");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct RunqueryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::RunQueryRequest,
        query_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RunqueryRequestBuilder<'a, A> {
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
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("query/");
            {
                let str_value = self.query_id.to_string();
                output.push_str(&str_value);
            }
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod reports {
    pub mod params {}
    pub struct ReportsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ReportsActions<'a, A> {
        #[doc = "Retrieves stored reports."]
        pub fn listreports(&self, query_id: i64) -> ListreportsRequestBuilder<A> {
            ListreportsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                query_id,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListreportsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        query_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListreportsRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::ListReportsResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("queries/");
            {
                let str_value = self.query_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/reports");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod sdf {
    pub mod params {}
    pub struct SdfActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> SdfActions<'a, A> {
        #[doc = "Retrieves entities in SDF format."]
        pub fn download(
            &self,
            request: crate::schemas::DownloadRequest,
        ) -> DownloadRequestBuilder<A> {
            DownloadRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DownloadRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::DownloadRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DownloadRequestBuilder<'a, A> {
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DownloadResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/doubleclickbidmanager/v1/".to_owned();
            output.push_str("sdf/download");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/doubleclickbidmanager"])
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
