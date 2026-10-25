create table if not exists weather_reading
(
    created_at      text not null,
    temperature_c   real not null,
    humidity_pct    real not null,
    pressure_pascal real not null
) strict;