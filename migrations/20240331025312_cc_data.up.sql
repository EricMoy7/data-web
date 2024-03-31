-- Add up migration script here
create table prepaid_cards (
    card_number VARCHAR not null primary key,
    expiration_month INT not null,
    expiration_year INT not null,
    security_code VARCHAR not null,
    current_amount FLOAT
);

create unique index card_number on prepaid_cards (card_number);

create table cc_transactions (
    id VARCHAR not null primary key,
    card_number VARCHAR not null,
    amount FLOAT not null,
    transaction_date TIMESTAMP not null,
    transaction_type VARCHAR not null,
    merchant_description VARCHAR not null,
    FOREIGN KEY (card_number) REFERENCES prepaid_cards(card_number)
);

create unique index id on cc_transactions (id);