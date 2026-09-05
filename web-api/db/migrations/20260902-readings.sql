create table reading
(
    epoch_timestamp  integer(8) not null,
    temperature_c    real       not null,
    pressure_mbar    real       not null,
    humidity_pct     real       not null,
    pm_25_ug_cubic_m real       not null
);