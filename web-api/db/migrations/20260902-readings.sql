create table reading
(
    epoch_timestamp integer(8) not null,
    temperature_c   real       not null,
    humidity_pct    real       not null,
    pm10_standard   integer(8) not null,
    pm25_standard   integer(8) not null,
    pm100_standard  integer(8) not null,
    pm10_env        integer(8) not null,
    pm25_env        integer(8) not null,
    pm100_env       integer(8) not null,
    particles_03um  integer(8) not null,
    particles_05um  integer(8) not null,
    particles_10um  integer(8) not null,
    particles_25um  integer(8) not null,
    particles_50um  integer(8) not null,
    particles_100um integer(8) not null
);