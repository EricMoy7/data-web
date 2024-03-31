use axum::{
    extract::State,http::HeaderMap, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};

use crate::blackhawk::
    {   database::DbConfig, 
        structures::{CardDetailsDb, TransactionQuery}, 
        visa_queries::{get_balance, get_transactions}, 
        query_structs::transactions::Transaction
    };

use sqlx;

pub async fn api() {

    let db_config = DbConfig::new("postgres://postgres:12345@localhost:5432/postgres".to_string()).await;
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root` 
        .route("/", get(root))

        .route("/newCard", post(create_card))

        .route("/updateNullCards", get(update_cards))

        .route("/getTransactions", post(update_transactions))
        // Passing DB Pool instances over to handlers
        .with_state(db_config);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_card(
    State(db): State<DbConfig>,
    Json(payload): Json<CardDetailsDb>
) -> Response {

    // TODO: Need some kind of validation

    let db_res = db.write_cc_info(&payload).await;

    match db_res {
        Ok(_n) => {
            let number = payload.card_number;
            if number.len() >= 4 {
                let last_four: String = number.chars().skip(number.len() - 4).collect();
                format!("Created entry for card {}", last_four).into_response()
            } else {
                "Created entry, but could not parse last 4 digits of card".into_response()
            }
        },
        Err(e) => e.as_database_error().unwrap().message().to_owned().into_response(),
        _ => "Unknown error has occured".into_response()
        }
}

async fn update_cards(
    db: State<DbConfig>,
    headers: HeaderMap,
) -> Response {

    let query_null = sqlx::query_as!(
        CardDetailsDb,
        "SELECT * FROM prepaid_cards WHERE current_amount IS NULL"
    );

    let query_positive = sqlx::query_as!(
        CardDetailsDb,
        "SELECT * FROM prepaid_cards WHERE current_amount > 0"
    );

    let db_res: Result<Vec<CardDetailsDb>, sqlx::Error>;

    if headers.contains_key("queryOnlyNull") {
        let query_only_null_value = headers.get("queryOnlyNull").unwrap().to_str().unwrap();
        if query_only_null_value == "true" {
            db_res = query_null.fetch_all(&db.pool).await;
        } else if query_only_null_value == "false" {
            db_res = query_positive.fetch_all(&db.pool).await;
        } else {
            // TODO
            return "You did not specify queryOnlyNull field or it was incorrect, use true or false (str)".into_response()
        }
    } else {
        return "Headers are empty for request!".into_response()
    }
        


    let mut batch: Vec<CardDetailsDb> = Vec::new();
    let mut error_count: i32 = 0;
    let mut success_count: i32 = 0;

    match db_res {
        Ok(cards) => {
            for card in cards {
                
                let res = get_balance(card.clone()).await.unwrap();

                // batch.push(serde_json::from_value(res.clone()).unwrap());
                let query = sqlx::query("UPDATE prepaid_cards SET current_amount = $1 WHERE card_number = $2")
                    .bind(res.get("result")
                        .and_then(|v| v.get("balances")
                        .and_then(|v| v.get("closingBalance")))
                        .and_then(|v| v.as_f64())
                        .unwrap()
                        // .as_f64()
                    )
                    .bind(card.card_number.to_string());

                query.execute(&db.pool).await.expect("One of the queries broke");
                success_count += 1;
            }
            format!("Successes: {} \n Errors: {}", success_count, error_count).to_string().into_response()
        },
        // serde_json::to_string(&n).expect("Serialization into JSON failed").into_response(),
        Err(e) => e.to_string().into_response()
    }
}


async fn update_transactions(
    db: State<DbConfig>,
    headers: HeaderMap,
    payload: Json<TransactionQuery>
) -> Response {
    let query_null = sqlx::query_as!(
        CardDetailsDb,
        "SELECT * FROM prepaid_cards WHERE current_amount IS NULL"
    );

    let query_positive = sqlx::query_as!(
        CardDetailsDb,
        "SELECT * FROM prepaid_cards WHERE current_amount > 0"
    );

    let db_res: Result<Vec<CardDetailsDb>, sqlx::Error>;

    if headers.contains_key("queryOnlyNull") {
        let query_only_null_value = headers.get("queryOnlyNull").unwrap().to_str().unwrap();
        if query_only_null_value == "true" {
            db_res = query_null.fetch_all(&db.pool).await;
        } else if query_only_null_value == "false" {
            db_res = query_positive.fetch_all(&db.pool).await;
        } else {
            // TODO
            return "You did not specify queryOnlyNull field or it was incorrect, use true or false (str)".into_response()
        }
    } else {
        return "Headers are empty for request!".into_response()
    }

    let mut batch: Vec<CardDetailsDb> = Vec::new();
    let mut error_count: i32 = 0;
    let mut success_count: i32 = 0;


    match db_res {
        Ok(cards) => {
            for card in cards {

                // The process involves calling get_balance to aquire the token necessary for transaction data
                let balance_sum_res = get_balance(card.clone()).await;
                let balance_sum = balance_sum_res.unwrap();
                let access_token = balance_sum.get("access_token").unwrap();
                let token = access_token.as_str().unwrap();
            

                let res = get_transactions(&payload, &token).await.unwrap(); 

                let trans_details= res.get("result").unwrap();
                let trans_struct: Transaction  = serde_json::from_value(trans_details.clone()).unwrap();
                let trans_vec = trans_struct.transactions;

                println!("{:#?}", trans_vec);

                for transaction in trans_vec {
                    let timestamp = transaction.transaction_date.replace("T", " ").replace("Z", "");
                    let req = sqlx::query(
                        r#"
                            INSERT INTO cc_transactions (id, card_number, amount, merchant_description, transaction_date, transaction_type)
                            VALUES ($1, $2, $3, $4, TO_TIMESTAMP($5, 'YYYY-MM-DD HH24:MI:SS.US'), $6)
                        "#
                    )
                    .bind(transaction.id)
                    .bind(card.card_number.clone())
                    .bind(transaction.amount)
                    .bind(transaction.merchant_description)
                    .bind(timestamp)
                    .bind(transaction.transaction_type);

                    // println!("{:#?}", req.unwrap());

                    let res = req.execute(&db.pool).await;
                    println!("{:#?}", res.unwrap())
                }

                

            }

            format!("Successes: {} \n Errors: {}", success_count, error_count).to_string().into_response()
        },
        // serde_json::to_string(&n).expect("Serialization into JSON failed").into_response(),
        Err(e) => e.to_string().into_response()
    }
}
