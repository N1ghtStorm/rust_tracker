create database if not exists rust_tracker;

create table if not exists events 
(
    Uuid String,
    Date String,
    Action String,
    Button String,
    ServerTimestamp String
) 
engine = MergeTree()
    PARTITION BY Date
    ORDER BY (Uuid)
    ;

create table if not exists kafka_events 
(
    Uuid String,
    Date String,
    Action String,
    Button String,
    ServerTimestamp String
) 
engine = Kafka()
    settings
        kafka_broker_list = 'kafka:19092',
        kafka_topic_list = 'facade-inbox',
        kafka_group_name = 'tracker-clickhouse',
        kafka_format = 'JSONEachRow',
        kafka_num_consumers = 1
        ;


create MATERIALIZED VIEW mv_events to events as
    select * from kafka_events ORDER by Uuid
    ;