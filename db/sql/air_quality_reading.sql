create table if not exists air_quality_reading
(
    created_at           text       not null,
    pm1_0_standard_ug_m3 integer(8) not null,
    pm2_5_standard_ug_m3 integer(8) not null,
    pm10_standard_ug_m3  integer(8) not null,
    pm1_0_env_ug_m3      integer(8) not null,
    pm2_5_env_ug_m3      integer(8) not null,
    pm10_env_ug_m3       integer(8) not null,
    particles_0_3um      integer(8) not null,
    particles_0_5um      integer(8) not null,
    particles_1_0um      integer(8) not null,
    particles_2_5um      integer(8) not null,
    particles_5_0um      integer(8) not null,
    particles_10um       integer(8) not null
) strict;