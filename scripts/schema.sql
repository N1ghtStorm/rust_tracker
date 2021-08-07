create database if not exists rust_tracker;

create table if not exists events 
(
    date Date,
    uuid UUID,
    timestamp DateTime
) 
engine = MergeTree()
    PARTITION BY date
    ORDER BY (uuid)