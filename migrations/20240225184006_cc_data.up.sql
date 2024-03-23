-- Add up migration script here
create table prepaid_cards (
    card_number VARCHAR not null,
    expiration_month INT not null,
    expiration_year INT not null,
    security_code VARCHAR not null,
    current_amount FLOAT 
);

create unique index card_number on prepaid_cards (card_number);