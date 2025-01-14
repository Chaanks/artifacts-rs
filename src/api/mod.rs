pub mod schema;

use reqwest::{Client, Method};
use schema::{
    r#type::{
        character::Character,
        item::{Item, ItemCode, ItemComponent, ItemSlot, Resource},
        map::Map,
        monster::Monster,
    },
    response::{
        my_account::AccountBankDetailsData,
        my_characters::{
            CharacterCraftData, CharacterEquipData, CharacterFightData, CharacterGatherData,
            CharacterGoldTransactionData, CharacterItemTransactionData, CharacterMovementData,
            CharacterRecycleData, CharacterRestData, CharacterUseItemData,
        },
        ResponseError, StatusData,
    },
    PagedSchemaWrapper, SchemaWrapper,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use thiserror::Error;

pub struct HttpRequest<'a> {
    pub path: String,
    pub data: Option<Value>,
    pub query: Option<Vec<(&'a str, &'a str)>>,
    pub method: Method,
}

pub trait ApiRequest {
    fn to_request(&self, name: &str) -> HttpRequest;
}

pub struct Api {
    token: String,
    host: String,
    client: Client,
}

impl Api {
    const HOST: &str = "https://api.artifactsmmo.com";

    pub fn new(token: String) -> Self {
        Self {
            token,
            host: Self::HOST.to_string(),
            client: Client::new(),
        }
    }

    pub async fn send<T: DeserializeOwned>(
        &self,
        request: HttpRequest<'_>,
    ) -> Result<T, ResponseError> {
        let url = format!("{}{}", self.host, request.path);

        let req = match request.method {
            Method::POST => self.client.post(&url),
            Method::GET => self.client.get(&url),
            _ => unimplemented!(),
        };

        let mut req = req
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {}", self.token));

        if let Some(data) = request.data {
            req = req.header("Content-Type", "application/json").json(&data);
        }

        if let Some(query) = request.query {
            req = req.query(&query);
        }

        let response = req.send().await.expect("Failed to send request");
        match response.status().as_u16() {
            200 => {
                let response_text = response
                    .text()
                    .await
                    .map_err(|_| ApiError::ApiError)
                    .expect("Failed to get full reponse text");
                let response_wrapped = serde_json::from_str::<SchemaWrapper<T>>(&response_text)
                    .expect("Failed to deserialize");
                Ok(response_wrapped.data)
            }
            _ => {
                let response_text = response
                    .text()
                    .await
                    .expect("Failed to get full reponse text");
                let err = serde_json::from_str::<ResponseError>(&response_text)
                    .expect("Failed to deserialize");
                Err(err)
            }
        }
    }

    pub async fn send_paginated<T: DeserializeOwned>(
        &self,
        request: HttpRequest<'_>,
    ) -> Result<Vec<T>, ResponseError> {
        let mut results = Vec::new();
        let mut page: usize = 1;

        let base_query = request.query.unwrap_or_default();

        loop {
            let mut query = base_query.clone();

            let page_str = page.to_string();
            let query_value = ("page", page_str.as_ref());
            query.push(query_value);

            let request_with_pagination = HttpRequest {
                path: request.path.clone(),
                data: request.data.clone(),
                query: Some(query),
                method: request.method.clone(),
            };

            let url = format!("{}{}", self.host, request_with_pagination.path);

            let req = match request_with_pagination.method {
                Method::POST => self.client.post(&url),
                Method::GET => self.client.get(&url),
                _ => unimplemented!(),
            };

            let mut req = req
                .header("Accept", "application/json")
                .header("Authorization", &format!("Bearer {}", self.token));

            if let Some(data) = request_with_pagination.data {
                req = req.header("Content-Type", "application/json").json(&data);
            }

            if let Some(query) = request_with_pagination.query {
                req = req.query(&query);
            }

            let response = req.send().await.expect("Failed to send request");

            match response.status().as_u16() {
                200 => {
                    let response_text = response
                        .text()
                        .await
                        .map_err(|_| ApiError::ApiError)
                        .expect("Failed to get full response text");

                    let page_data: PagedSchemaWrapper<Vec<T>> =
                        serde_json::from_str(&response_text).expect("Failed to deserialize");

                    results.extend(page_data.data);

                    if page >= page_data.pages as usize {
                        return Ok(results);
                    } else {
                        page += 1;
                    }
                }
                _ => {
                    let response_text = response
                        .text()
                        .await
                        .expect("Failed to get full response text");
                    let err = serde_json::from_str::<ResponseError>(&response_text)
                        .expect("Failed to deserialize");
                    return Err(err);
                }
            }
        }
    }

    pub async fn status(&self) -> Result<StatusData, ResponseError> {
        self.send(HttpRequest {
            path: "/".into(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    pub async fn character(&self, name: &str) -> Result<Character, ResponseError> {
        self.send(HttpRequest {
            path: format!("/characters/{name}"),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    /* My characters */
    pub async fn action_move(
        &self,
        name: &str,
        x: i32,
        y: i32,
    ) -> Result<CharacterMovementData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/move"),
            data: Some(json!({ "x": x, "y": y })),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_rest(&self, name: &str) -> Result<CharacterRestData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/rest"),
            data: None,
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_equip_item(
        &self,
        name: &str,
        code: &String,
        slot: &ItemSlot,
        quantity: u32,
    ) -> Result<CharacterEquipData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/equip"),
            data: Some(json!({ "code": code, "slot": slot, "quantity": quantity })),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_unequip_item(
        &self,
        name: &str,
        slot: &ItemSlot,
        quantity: u32,
    ) -> Result<CharacterEquipData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/unequip"),
            data: Some(json!({ "slot": slot, "quantity": quantity })),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_use_item(
        &self,
        name: &str,
        code: &ItemCode,
        quantity: u32,
    ) -> Result<CharacterUseItemData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/use"),
            data: Some(json!({ "code": code, "quantity": quantity })),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_fight(&self, name: &str) -> Result<CharacterFightData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/fight"),
            data: Some(json!({})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_gather(&self, name: &str) -> Result<CharacterGatherData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/gathering"),
            data: Some(json!({})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_craft(
        &self,
        name: &str,
        code: &ItemCode,
        quantity: u32,
    ) -> Result<CharacterCraftData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/crafting"),
            data: Some(json!({"code": code, "quantity": quantity})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_recycle(
        &self,
        name: &str,
        code: &ItemCode,
        quantity: u32,
    ) -> Result<CharacterRecycleData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/recycling"),
            data: Some(json!({"code": code, "quantity": quantity})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_deposit_gold(
        &self,
        name: &str,
        quantity: u32,
    ) -> Result<CharacterGoldTransactionData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/bank/deposit/gold"),
            data: Some(json!({"quantity": quantity})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_withdraw_gold(
        &self,
        name: &str,
        quantity: u32,
    ) -> Result<CharacterGoldTransactionData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/bank/withdraw/gold"),
            data: Some(json!({"quantity": quantity})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_deposit_item(
        &self,
        name: &str,
        code: &ItemCode,
        quantity: u32,
    ) -> Result<CharacterItemTransactionData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/bank/deposit"),
            data: Some(json!({"code": code, "quantity": quantity})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    pub async fn action_withdraw_item(
        &self,
        name: &str,
        code: &ItemCode,
        quantity: u32,
    ) -> Result<CharacterItemTransactionData, ResponseError> {
        self.send(HttpRequest {
            path: format!("/my/{name}/action/bank/withdraw"),
            data: Some(json!({"code": code, "quantity": quantity})),
            query: None,
            method: Method::POST,
        })
        .await
    }

    /* My account */
    pub async fn bank_items(&self) -> Result<Vec<ItemComponent>, ResponseError> {
        self.send(HttpRequest {
            path: "/my/bank/items".to_string(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    pub async fn bank_details(&self) -> Result<AccountBankDetailsData, ResponseError> {
        self.send(HttpRequest {
            path: "/my/bank".to_string(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    /* Items */
    pub async fn items(&self) -> Result<Vec<Item>, ResponseError> {
        self.send_paginated(HttpRequest {
            path: "/items".to_string(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    pub async fn item(&self, code: &ItemCode) -> Result<Item, ResponseError> {
        self.send(HttpRequest {
            path: format!("/maps/{code}"),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    /* Resources */
    pub async fn resources_drop(&self, drop: &ItemCode) -> Result<Vec<Resource>, ResponseError> {
        self.send(HttpRequest {
            path: "/resources".to_string(),
            data: None,
            query: Some(vec![("drop", drop)]),
            method: Method::GET,
        })
        .await
    }

    pub async fn resources(&self) -> Result<Vec<Resource>, ResponseError> {
        self.send_paginated(HttpRequest {
            path: "/resources".to_string(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    pub async fn resource(&self, code: &ItemCode) -> Result<Resource, ResponseError> {
        self.send(HttpRequest {
            path: format!("/resources/{code}"),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    /* Maps */
    pub async fn maps(&self) -> Result<Vec<Map>, ResponseError> {
        self.send_paginated(HttpRequest {
            path: "/maps".to_string(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    pub async fn maps_content(
        &self,
        content_code: &String,
        content_type: &String,
    ) -> Result<Vec<Map>, ResponseError> {
        self.send(HttpRequest {
            path: "/maps".to_string(),
            data: None,
            query: Some(vec![
                ("content_code", content_code),
                ("content_type", content_type),
            ]),
            method: Method::GET,
        })
        .await
    }

    pub async fn map(&self, x: i32, y: i32) -> Result<Map, ResponseError> {
        self.send(HttpRequest {
            path: format!("/maps/{x}/{y}"),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    /* Monsters */
    pub async fn monster(&self, monster: &str) -> Result<Monster, ResponseError> {
        self.send(HttpRequest {
            path: format!("/monsters/{monster}"),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }

    pub async fn monsters(&self) -> Result<Vec<Monster>, ResponseError> {
        self.send_paginated(HttpRequest {
            path: "/monsters".to_string(),
            data: None,
            query: None,
            method: Method::GET,
        })
        .await
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Api not found")]
    ApiError,
    #[error("{0}")]
    ArtifactsError(ResponseError),

    #[error("Unexpected response type")]
    UnexpectedApiResponseType,

    #[error("Deserialization error")]
    Deserialization(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown(u16),
}
