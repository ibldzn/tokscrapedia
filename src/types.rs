use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct SearchResult {
    pub header: Header,
    pub data: Data,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Data {
    pub source: String,
    pub share_url: String,
    pub q: String,
    pub is_filter: bool,
    pub is_query_safe: bool,
    pub suggestion_text: SuggestionText,
    pub ticker: Ticker,
    pub redirection: Redirection,
    pub related: Related,
    pub suggestions: Suggestions,
    pub suggestions_instead: SuggestionsInstead,
    pub catalogs: Vec<Option<serde_json::Value>>,
    pub products: Vec<Product>,
    pub default_search_url: String,
    pub autocomplete_applink: String,
    pub navsource: String,
    pub lite_url: String,
    pub banner: Banner,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Banner {
    pub position: i64,
    pub text: String,
    pub applink: String,
    pub image_url: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub childs: Option<Vec<i64>>,
    pub url: String,
    pub image_url: String,
    pub image_url_700: String,
    pub image_url_500: String,
    pub image_url_300: String,
    pub price: String,
    pub price_range: String,
    pub shop: Shop,
    pub wholesale_price: Vec<Option<serde_json::Value>>,
    pub courier_count: i64,
    pub condition: i64,
    pub category_id: i64,
    pub category_name: String,
    pub category_breadcrumb: String,
    pub department_id: i64,
    pub department_name: String,
    pub labels: Vec<Label>,
    pub label_groups: Vec<LabelGroup>,
    pub label_group_variant: Option<serde_json::Value>,
    pub free_ongkir: FreeOngkir,
    pub top_label: Option<Vec<String>>,
    pub bottom_label: Option<serde_json::Value>,
    pub badges: Vec<Badge>,
    pub is_featured: i64,
    pub rating: i64,
    pub count_review: i64,
    pub price_int: i64,
    pub parent_id: i64,
    pub original_price: String,
    pub discount_expired: String,
    pub discount_start: String,
    pub discount_percentage: i64,
    pub sku: String,
    pub stock: i64,
    pub status: i64,
    pub ga_key: String,
    pub is_preorder: bool,
    pub min_order: i64,
    pub warehouse_id_default: i64,
    pub rating_average: String,
    pub cluster_id: String,
    pub source_engine: String,
    pub booster_list: String,
    pub applink: String,
    pub annotation_id: Option<Vec<String>>,
    pub count_sold: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Badge {
    pub title: String,
    pub image_url: String,
    pub show: bool,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct FreeOngkir {
    pub is_active: bool,
    pub img_url: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct LabelGroup {
    pub position: String,
    pub title: String,
    #[serde(rename = "type")]
    pub label_group_type: String,
    pub url: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Label {
    pub title: String,
    pub color: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Shop {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub shop_tier: i64,
    pub is_gold: bool,
    pub location: String,
    pub city: String,
    pub reputation: String,
    pub clover: String,
    pub is_official: bool,
    pub is_tokonow: bool,
    pub is_power_badge: bool,
    pub rating_average: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Redirection {
    pub redirect_url: String,
    pub redirect_applink: String,
    pub department_id: i64,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Related {
    pub related_keyword: String,
    pub position: i64,
    pub tracking_option: i64,
    pub other_related: Vec<Option<serde_json::Value>>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct SuggestionText {
    pub text: String,
    pub query: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Suggestions {
    pub suggestion: String,
    pub total_data: i64,
    pub component_id: String,
    pub tracking_option: i64,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct SuggestionsInstead {
    pub suggestion_instead: String,
    pub current_keyword: String,
    pub total_data: i64,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Ticker {
    pub text: String,
    pub query: String,
    pub applink: String,
    pub type_id: i64,
    pub component_id: String,
    pub tracking_option: i64,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Header {
    pub total_data: i64,
    pub total_data_text: String,
    pub default_view: i64,
    pub process_time: f64,
    pub additional_params: String,
    pub response_code: i64,
    pub error_message: String,
    pub keyword_process: String,
    pub component_id: String,
}
