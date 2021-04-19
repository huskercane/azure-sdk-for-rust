#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDefinition {
    #[serde(rename = "type")]
    pub type_: report_config_definition::Type,
    pub timeframe: report_config_definition::Timeframe,
    #[serde(rename = "timePeriod", skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ReportConfigTimePeriod>,
    #[serde(rename = "dataSet", skip_serializing_if = "Option::is_none")]
    pub data_set: Option<ReportConfigDataset>,
    #[serde(rename = "includeMonetaryCommitment", skip_serializing)]
    pub include_monetary_commitment: Option<bool>,
}
pub mod report_config_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigTimePeriod {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<report_config_dataset::Granularity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ReportConfigDatasetConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<ReportConfigGrouping>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sorting: Vec<ReportConfigSorting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
}
pub mod report_config_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
        Monthly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDatasetConfiguration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigAggregation {
    pub name: String,
    pub function: report_config_aggregation::Function,
}
pub mod report_config_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigSorting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<report_config_sorting::Direction>,
    pub name: String,
}
pub mod report_config_sorting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        Ascending,
        Descending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigGrouping {
    #[serde(rename = "type")]
    pub type_: ReportConfigColumnType,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigFilter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<ReportConfigFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<ReportConfigFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ReportConfigComparisonExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ReportConfigComparisonExpression>,
    #[serde(rename = "tagKey", skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<ReportConfigComparisonExpression>,
    #[serde(rename = "tagValue", skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<ReportConfigComparisonExpression>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReportConfigColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigComparisonExpression {
    pub name: String,
    pub operator: report_config_comparison_expression::Operator,
    pub values: Vec<String>,
}
pub mod report_config_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
        Contains,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Setting>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    #[serde(flatten)]
    pub proxy_setting_resource: ProxySettingResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SettingsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewListResult {
    #[serde(skip_serializing)]
    pub value: Vec<View>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct View {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ViewProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsProperties {
    pub scope: String,
    #[serde(rename = "startOn", skip_serializing_if = "Option::is_none")]
    pub start_on: Option<settings_properties::StartOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
}
pub mod settings_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StartOn {
        LastUsed,
        ScopePicker,
        SpecificScope,
    }
}
pub type Cache = Vec<serde_json::Value>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "createdOn", skip_serializing)]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", skip_serializing)]
    pub modified_on: Option<String>,
    #[serde(rename = "dateRange", skip_serializing)]
    pub date_range: Option<String>,
    #[serde(skip_serializing)]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<ReportConfigDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<view_properties::Chart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulated: Option<view_properties::Accumulated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<view_properties::Metric>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kpis: Vec<KpiProperties>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pivots: Vec<PivotProperties>,
}
pub mod view_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Chart {
        Area,
        Line,
        StackedColumn,
        GroupedColumn,
        Table,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Accumulated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Metric {
        ActualCost,
        AmortizedCost,
        #[serde(rename = "AHUB")]
        Ahub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiProperties {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<kpi_properties::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
pub mod kpi_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Forecast,
        Budget,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PivotProperties {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<pivot_properties::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub mod pivot_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Dimension,
        TagKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxySettingResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub kind: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Dimension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionProperties {
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "filterEnabled", skip_serializing)]
    pub filter_enabled: Option<bool>,
    #[serde(rename = "groupingEnabled", skip_serializing)]
    pub grouping_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,
    #[serde(skip_serializing)]
    pub total: Option<i32>,
    #[serde(skip_serializing)]
    pub category: Option<String>,
    #[serde(rename = "usageStart", skip_serializing)]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", skip_serializing)]
    pub usage_end: Option<String>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsResult {
    #[serde(skip_serializing)]
    pub value: Vec<Alert>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<alert_properties::Definition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<alert_properties::Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<alert_properties::Details>,
    #[serde(rename = "costEntityId", skip_serializing_if = "Option::is_none")]
    pub cost_entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<alert_properties::Status>,
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "closeTime", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(rename = "modificationTime", skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "statusModificationUserName", skip_serializing_if = "Option::is_none")]
    pub status_modification_user_name: Option<String>,
    #[serde(rename = "statusModificationTime", skip_serializing_if = "Option::is_none")]
    pub status_modification_time: Option<String>,
}
pub mod alert_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Definition {
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_: Option<definition::Type>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category: Option<definition::Category>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub criteria: Option<definition::Criteria>,
    }
    pub mod definition {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            Budget,
            Invoice,
            Credit,
            Quota,
            General,
            #[serde(rename = "xCloud")]
            XCloud,
            BudgetForecast,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Category {
            Cost,
            Usage,
            Billing,
            System,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Criteria {
            CostThresholdExceeded,
            UsageThresholdExceeded,
            CreditThresholdApproaching,
            CreditThresholdReached,
            QuotaThresholdApproaching,
            QuotaThresholdReached,
            MultiCurrency,
            ForecastCostThresholdExceeded,
            ForecastUsageThresholdExceeded,
            InvoiceDueDateApproaching,
            InvoiceDueDateReached,
            CrossCloudNewDataAvailable,
            CrossCloudCollectionError,
            GeneralThresholdError,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        Preset,
        User,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Details {
        #[serde(rename = "timeGrainType", skip_serializing_if = "Option::is_none")]
        pub time_grain_type: Option<details::TimeGrainType>,
        #[serde(rename = "periodStartDate", skip_serializing_if = "Option::is_none")]
        pub period_start_date: Option<String>,
        #[serde(rename = "triggeredBy", skip_serializing_if = "Option::is_none")]
        pub triggered_by: Option<String>,
        #[serde(rename = "resourceGroupFilter", skip_serializing_if = "Vec::is_empty")]
        pub resource_group_filter: Vec<serde_json::Value>,
        #[serde(rename = "resourceFilter", skip_serializing_if = "Vec::is_empty")]
        pub resource_filter: Vec<serde_json::Value>,
        #[serde(rename = "meterFilter", skip_serializing_if = "Vec::is_empty")]
        pub meter_filter: Vec<serde_json::Value>,
        #[serde(rename = "tagFilter", skip_serializing_if = "Option::is_none")]
        pub tag_filter: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub threshold: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operator: Option<details::Operator>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(rename = "currentSpend", skip_serializing_if = "Option::is_none")]
        pub current_spend: Option<f64>,
        #[serde(rename = "contactEmails", skip_serializing_if = "Vec::is_empty")]
        pub contact_emails: Vec<String>,
        #[serde(rename = "contactGroups", skip_serializing_if = "Vec::is_empty")]
        pub contact_groups: Vec<String>,
        #[serde(rename = "contactRoles", skip_serializing_if = "Vec::is_empty")]
        pub contact_roles: Vec<String>,
        #[serde(rename = "overridingAlert", skip_serializing_if = "Option::is_none")]
        pub overriding_alert: Option<String>,
    }
    pub mod details {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TimeGrainType {
            None,
            Monthly,
            Quarterly,
            Annually,
            BillingMonth,
            BillingQuarter,
            BillingAnnual,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Operator {
            None,
            EqualTo,
            GreaterThan,
            GreaterThanOrEqualTo,
            LessThan,
            LessThanOrEqualTo,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        None,
        Active,
        Overridden,
        Resolved,
        Dismissed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DismissAlertPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryProperties {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<QueryColumn>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryColumn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastDefinition {
    #[serde(rename = "type")]
    pub type_: forecast_definition::Type,
    pub timeframe: forecast_definition::Timeframe,
    #[serde(rename = "timePeriod", skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<QueryDataset>,
    #[serde(rename = "includeActualCost", skip_serializing_if = "Option::is_none")]
    pub include_actual_cost: Option<bool>,
    #[serde(rename = "includeFreshPartialCost", skip_serializing_if = "Option::is_none")]
    pub include_fresh_partial_cost: Option<bool>,
}
pub mod forecast_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDefinition {
    #[serde(rename = "type")]
    pub type_: query_definition::Type,
    pub timeframe: query_definition::Timeframe,
    #[serde(rename = "timePeriod", skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<QueryDataset>,
}
pub mod query_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTimePeriod {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<query_dataset::Granularity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<QueryDatasetConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<QueryGrouping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<QueryFilter>,
}
pub mod query_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<status::Status>,
}
pub mod status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Running,
        Completed,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReportUrl>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportUrl {
    #[serde(rename = "reportUrl", skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDatasetConfiguration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryAggregation {
    pub name: String,
    pub function: query_aggregation::Function,
}
pub mod query_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryGrouping {
    #[serde(rename = "type")]
    pub type_: QueryColumnType,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFilter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<QueryFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<QueryFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<QueryComparisonExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<QueryComparisonExpression>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryComparisonExpression {
    pub name: String,
    pub operator: query_comparison_expression::Operator,
    pub values: Vec<String>,
}
pub mod query_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDefinition {
    #[serde(rename = "type")]
    pub type_: export_definition::Type,
    pub timeframe: export_definition::Timeframe,
    #[serde(rename = "timePeriod", skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    #[serde(rename = "dataSet", skip_serializing_if = "Option::is_none")]
    pub data_set: Option<QueryDataset>,
}
pub mod export_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Export>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Export {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportProperties {
    #[serde(flatten)]
    pub common_export_properties: CommonExportProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ExportSchedule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonExportProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<common_export_properties::Format>,
    #[serde(rename = "deliveryInfo")]
    pub delivery_info: ExportDeliveryInfo,
    pub definition: ExportDefinition,
}
pub mod common_export_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Csv,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<export_schedule::Status>,
    pub recurrence: export_schedule::Recurrence,
    #[serde(rename = "recurrencePeriod", skip_serializing_if = "Option::is_none")]
    pub recurrence_period: Option<ExportRecurrencePeriod>,
}
pub mod export_schedule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Inactive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        Daily,
        Weekly,
        Monthly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryInfo {
    pub destination: ExportDeliveryDestination,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportRecurrencePeriod {
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryDestination {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub container: String,
    #[serde(rename = "rootFolderPath", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportExecutionListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ExportExecution>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportExecution {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportExecutionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportExecutionProperties {
    #[serde(rename = "executionType", skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<export_execution_properties::ExecutionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<export_execution_properties::Status>,
    #[serde(rename = "submittedBy", skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(rename = "submittedTime", skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<String>,
    #[serde(rename = "processingStartTime", skip_serializing_if = "Option::is_none")]
    pub processing_start_time: Option<String>,
    #[serde(rename = "processingEndTime", skip_serializing_if = "Option::is_none")]
    pub processing_end_time: Option<String>,
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "runSettings", skip_serializing_if = "Option::is_none")]
    pub run_settings: Option<CommonExportProperties>,
}
pub mod export_execution_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionType {
        OnDemand,
        Scheduled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Queued,
        InProgress,
        Completed,
        Failed,
        Timeout,
        NewDataNotAvailable,
        DataNotAvailable,
    }
}
