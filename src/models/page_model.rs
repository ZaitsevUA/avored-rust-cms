use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use super::{BaseModel, Pagination};

// // This one should contain components and components fields with content
// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct ComponentContentModel {
//     pub name: String,
//     pub identifier: String,
//     pub elements: Vec<ComponentElementModel>,
// }

// #[derive(Deserialize, Debug, Clone, Serialize)]
// #[serde(untagged)]
// pub enum PageElementContentDataType {
//     StringType(String),
//     Int64(i64)
// }

// impl Default for PageElementContentDataType {
//     fn default() -> PageElementContentDataType {
//         PageElementContentDataType::StringType("".to_string())
//     }
// }

// #[derive(Deserialize, Debug, Clone, Default, Serialize)]
// pub struct ComponentElementModel {
//     pub name: String,
//     pub identifier: String,
//     pub element_type: String,
//     pub element_data_type: String,
//     pub element_content: PageElementContentDataType,
//     pub element_data: Vec<PageComponentFieldDataOption>
// }

// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct PageComponentFieldDataOption {
//     pub label: String,
//     pub value: String
// }

// #[derive(Serialize, Debug, Deserialize, Clone, Default)]
// pub struct PageModel {
//     pub id: String,
//     pub name: String,
//     pub identifier: String,
//     pub components_content: Vec<ComponentContentModel>,
//     pub created_at: Datetime,
//     pub updated_at: Datetime,
//     pub created_by: String,
//     pub updated_by: String,
// }


#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct NewPageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub page_fields: Vec<PageFieldModel>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageFieldModel {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub field_content: String,
}

// impl TryFrom<Object> for PageModel {
//     type Error = Error;
//     fn try_from(val: Object) -> Result<PageModel> {
//
//         let id = val.get("id").get_id()?;
//         let name = val.get("name").get_string()?;
//         let identifier = val.get("identifier").get_string()?;
//
//         let components_content = match val.get("components_content") {
//             Some(val) => {
//
//                 match val.clone() {
//                     Value::Array(v) => {
//                         let mut arr = Vec::new();
//
//                         for array in v.into_iter() {
//                             let object = match array.clone() {
//                                 Value::Object(v) => v,
//                                 _ => surrealdb::sql::Object::default(),
//                             };
//
//                             let order_product_model: ComponentContentModel = object.try_into()?;
//
//                             arr.push(order_product_model)
//                         }
//                         arr
//                     }
//                     _ => Vec::new(),
//                 }
//             }
//             None => Vec::new(),
//         };
//
//         let created_at = val.get("created_at").get_datetime()?;
//         let updated_at = val.get("updated_at").get_datetime()?;
//         let created_by = val.get("created_by").get_string()?;
//         let updated_by = val.get("updated_by").get_string()?;
//
//         Ok(PageModel {
//             id,
//             name,
//             identifier,
//             components_content,
//             created_at,
//             updated_at,
//             created_by,
//             updated_by,
//         })
//     }
// }

impl TryFrom<Object> for NewPageModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<NewPageModel> {

        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;

        let page_fields = match val.get("page_fields") {
                        Some(val) => {

                            match val.clone() {
                                Value::Array(v) => {
                                    let mut arr = Vec::new();

                                    for array in v.into_iter() {
                                        let object = match array.clone() {
                                            Value::Object(v) => v,
                                            _ => surrealdb::sql::Object::default(),
                                        };

                                        let page_field: PageFieldModel = object.try_into()?;

                                        arr.push(page_field)
                                    }
                                    arr
                                }
                                _ => Vec::new(),
                            }
                        }
                        None => Vec::new(),
                    };


        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(NewPageModel {
            id,
            name,
            identifier,
            page_fields,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

impl TryFrom<Object> for PageFieldModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageFieldModel> {
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let data_type = val.get("data_type").get_string()?;
        let field_type = val.get("field_type").get_string()?;
        let field_content = val.get("field_content").get_string()?;

        Ok(PageFieldModel {
            name,
            identifier,
            data_type,
            field_type,
            field_content,
        })
    }
}
//
// impl TryFrom<Object> for ComponentElementModel {
//     type Error = Error;
//     fn try_from(val: Object) -> Result<ComponentElementModel> {
//
//         let name = val.get("name").get_string()?;
//         let identifier = val.get("identifier").get_string()?;
//         let element_type = val.get("element_type").get_string()?;
//         let element_data_type = val.get("element_data_type").get_string()?;
//         // let element_content = val.get("element_content").get_string()?;
//
//         let element_content = match element_data_type.as_str() {
//             "TEXT" => {
//                 let value = val.get("element_content").get_string()?;
//
//                 PageElementContentDataType::StringType(value)
//             },
//             "INT" => {
//                 let value = val.get("element_content").get_int()?;
//                 PageElementContentDataType::Int64(value)
//             },
//             _ => PageElementContentDataType::default()
//         };
//
//         // let element_data: Vec<PageComponentFieldDataOption> = val.get("element_data").get_array()?;
//         let element_data = match val.get("element_data") {
//             Some(val) => {
//
//                 match val.clone() {
//                     Value::Array(v) => {
//                         let mut arr = Vec::new();
//
//                         for array in v.into_iter() {
//                             let object = match array.clone() {
//                                 Value::Object(v) => v,
//                                 _ => surrealdb::sql::Object::default(),
//                             };
//
//                             let field_data_option: PageComponentFieldDataOption = object.try_into()?;
//
//                             arr.push(field_data_option)
//                         }
//                         arr
//                     }
//                     _ => Vec::new(),
//                 }
//             }
//             None => Vec::new(),
//         };
//
//         Ok(ComponentElementModel {
//             name,
//             identifier,
//             element_type,
//             element_data_type,
//             element_content,
//             element_data
//         })
//     }
// }
//
// impl TryFrom<Object> for PageComponentFieldDataOption {
//     type Error = Error;
//     fn try_from(val: Object) -> Result<PageComponentFieldDataOption> {
//         let label = val.get("label").get_string()?;
//         let value = val.get("value").get_string()?;
//
//         Ok(PageComponentFieldDataOption {
//             label,
//             value
//         })
//     }
// }

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PagePagination {
    pub data: Vec<NewPageModel>,
    pub pagination: Pagination,
}



// #[derive(Serialize, Debug, Deserialize, Clone)]
// pub struct CreatablePageModel {
//     pub name: String,
//     pub identifier: String,
//     pub logged_in_username: String,
//     pub component_contents: Vec<CreatableComponentContentModel>,
// }


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct NewCreatablePageModel {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub page_fields: Vec<CreatablePageField>
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub field_content: String
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct NewUpdatablePageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub page_fields: Vec<UpdatablePageField>
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub field_content: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutPageIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String
}

